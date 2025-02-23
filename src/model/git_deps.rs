use async_compression::tokio::bufread::GzipDecoder;
use futures_util::TryStreamExt;
use nom::Parser;
use nom::character::complete::multispace0;
use tokio::io;
use tokio::io::AsyncRead;
use tokio_util::io::StreamReader;

use crate::parser::*;

use super::{BlobMap, DependencyManifest, File, FileMap, PackMap};

pub struct GitDeps<'a> {
    dependency_manifest: DependencyManifest<'a>,
    files: FileMap<'a>,
    blobs: BlobMap<'a>,
    packs: PackMap<'a>,
}

impl<'a> GitDeps<'a> {
    pub fn new(input: &'a str) -> Result<Self, nom::Err<nom::error::Error<&'a str>>> {
        let (_, (_, _, (dependency_manifest, (files, _, blobs, _, packs)))) = (
            xml_header,
            multispace0,
            dependency_manifest((files, multispace0, blobs, multispace0, packs)),
        )
            .parse(input)?;

        Ok(GitDeps {
            dependency_manifest,
            files,
            blobs,
            packs,
        })
    }

    pub fn get_file(&self, file_name: &'a str) -> Option<File<'a>> {
        self.files.get(file_name)
    }

    pub fn get_file_url(&self, file: &File<'a>) -> Option<String> {
        let blob = self.blobs.get(file.hash())?;
        let pack = self.packs.get(blob.pack_hash())?;

        Some(format!(
            "{base_url}/{remote_path}/{pack_hash}",
            base_url = self.dependency_manifest.base_url(),
            remote_path = pack.remote_path(),
            pack_hash = blob.pack_hash()
        ))
    }

    pub async fn get_file_content(
        &self,
        file: &File<'a>,
    ) -> Result<impl AsyncRead, Box<dyn std::error::Error>> {
        let url = self.get_file_url(&file).ok_or("File not found")?;

        let response = reqwest::get(url).await?;
        let byte_stream = response
            .bytes_stream()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e));

        let stream_reader = StreamReader::new(byte_stream);
        let buf_reader = io::BufReader::new(stream_reader);

        let decoder = GzipDecoder::new(buf_reader);

        Ok(decoder)
    }
}
