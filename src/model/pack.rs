use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Pack<'a> {
    hash: &'a str,
    size: u32,
    compressed_size: u32,
    remote_path: &'a str,
}

impl<'a> Pack<'a> {
    pub fn new(hash: &'a str, size: u32, compressed_size: u32, remote_path: &'a str) -> Self {
        Self {
            hash,
            size,
            compressed_size,
            remote_path,
        }
    }

    pub fn hash(&self) -> &str {
        self.hash
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn compressed_size(&self) -> u32 {
        self.compressed_size
    }

    pub fn remote_path(&self) -> &str {
        self.remote_path
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct PackKey<'a> {
    hash: &'a str,
}

#[derive(Debug, PartialEq)]
struct PackValue<'a> {
    size: u32,
    compressed_size: u32,
    remote_path: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct PackMap<'a> {
    inner: HashMap<PackKey<'a>, PackValue<'a>>,
}

impl<'a> PackMap<'a> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, pack: Pack<'a>) {
        self.inner.insert(
            PackKey { hash: pack.hash },
            PackValue {
                size: pack.size,
                compressed_size: pack.compressed_size,
                remote_path: pack.remote_path,
            },
        );
    }

    pub fn get(&self, hash: &'a str) -> Option<Pack<'a>> {
        self.inner.get(&PackKey { hash }).map(|value| Pack {
            hash,
            size: value.size,
            compressed_size: value.compressed_size,
            remote_path: value.remote_path,
        })
    }
}
