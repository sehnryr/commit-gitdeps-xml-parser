use std::collections::HashMap;

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

#[derive(Debug, PartialEq, Eq, Hash)]
struct FileKey<'a> {
    name: &'a str,
}

#[derive(Debug, PartialEq)]
struct FileValue<'a> {
    hash: &'a str,
    is_executable: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileMap<'a> {
    inner: HashMap<FileKey<'a>, FileValue<'a>>,
}

impl<'a> FileMap<'a> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, file: File<'a>) {
        self.inner.insert(
            FileKey { name: file.name },
            FileValue {
                hash: file.hash,
                is_executable: file.is_executable,
            },
        );
    }

    pub fn get(&self, name: &'a str) -> Option<File<'a>> {
        self.inner.get(&FileKey { name }).map(|value| File {
            name,
            hash: value.hash,
            is_executable: value.is_executable,
        })
    }
}
