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

    pub fn into_key_value(self) -> (PackKey<'a>, PackValue<'a>) {
        (
            PackKey { hash: self.hash },
            PackValue {
                size: self.size,
                compressed_size: self.compressed_size,
                remote_path: self.remote_path,
            },
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PackKey<'a> {
    hash: &'a str,
}

impl<'a> PackKey<'a> {
    pub fn new(hash: &'a str) -> Self {
        Self { hash }
    }

    pub fn hash(&self) -> &str {
        self.hash
    }
}

#[derive(Debug, PartialEq)]
pub struct PackValue<'a> {
    size: u32,
    compressed_size: u32,
    remote_path: &'a str,
}

impl<'a> PackValue<'a> {
    #[cfg(test)]
    pub fn new(size: u32, compressed_size: u32, remote_path: &'a str) -> Self {
        Self {
            size,
            compressed_size,
            remote_path,
        }
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
