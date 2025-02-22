#[derive(Debug, PartialEq)]
pub struct XmlHeader<'a> {
    version: &'a str,
    encoding: &'a str,
}

impl<'a> XmlHeader<'a> {
    pub fn new(version: &'a str, encoding: &'a str) -> Self {
        Self { version, encoding }
    }
}

#[derive(Debug, PartialEq)]
pub struct DependencyManifest<'a> {
    xml_schema_definition_namespace: &'a str,
    xml_schema_instance_namespace: &'a str,
    base_url: &'a str,
}

impl<'a> DependencyManifest<'a> {
    pub fn new(
        xml_schema_definition_namespace: &'a str,
        xml_schema_instance_namespace: &'a str,
        base_url: &'a str,
    ) -> Self {
        Self {
            xml_schema_definition_namespace,
            xml_schema_instance_namespace,
            base_url,
        }
    }

    pub fn base_url(&self) -> &str {
        self.base_url
    }
}

#[derive(Debug, PartialEq)]
pub struct File<'a> {
    name: &'a str,
    hash: &'a str,
    is_executable: bool,
}

impl<'a> File<'a> {
    pub fn new(name: &'a str, hash: &'a str, is_executable: bool) -> Self {
        Self {
            name,
            hash,
            is_executable,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn hash(&self) -> &str {
        self.hash
    }

    pub fn is_executable(&self) -> bool {
        self.is_executable
    }
}

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
