use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Blob<'a> {
    hash: &'a str,
    size: u32,
    pack_hash: &'a str,
    pack_offset: u32,
}

impl<'a> Blob<'a> {
    pub fn new(hash: &'a str, size: u32, pack_hash: &'a str, pack_offset: u32) -> Self {
        Self {
            hash,
            size,
            pack_hash,
            pack_offset,
        }
    }

    pub fn hash(&self) -> &str {
        self.hash
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn pack_hash(&self) -> &str {
        self.pack_hash
    }

    pub fn pack_offset(&self) -> u32 {
        self.pack_offset
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct BlobKey<'a> {
    hash: &'a str,
}

#[derive(Debug, PartialEq)]
struct BlobValue<'a> {
    size: u32,
    pack_hash: &'a str,
    pack_offset: u32,
}

#[derive(Debug, PartialEq)]
pub struct BlobMap<'a> {
    inner: HashMap<BlobKey<'a>, BlobValue<'a>>,
}

impl<'a> BlobMap<'a> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, blob: Blob<'a>) {
        self.inner.insert(
            BlobKey { hash: blob.hash },
            BlobValue {
                size: blob.size,
                pack_hash: blob.pack_hash,
                pack_offset: blob.pack_offset,
            },
        );
    }

    pub fn get(&self, hash: &'a str) -> Option<Blob<'a>> {
        self.inner.get(&BlobKey { hash }).map(|value| Blob {
            hash,
            size: value.size,
            pack_hash: value.pack_hash,
            pack_offset: value.pack_offset,
        })
    }
}
