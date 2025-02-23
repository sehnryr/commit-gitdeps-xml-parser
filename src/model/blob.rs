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

    pub fn into_key_value(self) -> (BlobKey<'a>, BlobValue<'a>) {
        (
            BlobKey { hash: self.hash },
            BlobValue {
                size: self.size,
                pack_hash: self.pack_hash,
                pack_offset: self.pack_offset,
            },
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BlobKey<'a> {
    hash: &'a str,
}

impl<'a> BlobKey<'a> {
    pub fn new(hash: &'a str) -> Self {
        Self { hash }
    }

    pub fn hash(&self) -> &str {
        self.hash
    }
}

#[derive(Debug, PartialEq)]
pub struct BlobValue<'a> {
    size: u32,
    pack_hash: &'a str,
    pack_offset: u32,
}

impl<'a> BlobValue<'a> {
    #[cfg(test)]
    pub fn new(size: u32, pack_hash: &'a str, pack_offset: u32) -> Self {
        Self {
            size,
            pack_hash,
            pack_offset,
        }
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
