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

    pub fn into_key_value(self) -> (FileKey<'a>, FileValue<'a>) {
        (
            FileKey { name: self.name },
            FileValue {
                hash: self.hash,
                is_executable: self.is_executable,
            },
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FileKey<'a> {
    name: &'a str,
}

impl<'a> FileKey<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

#[derive(Debug, PartialEq)]
pub struct FileValue<'a> {
    hash: &'a str,
    is_executable: bool,
}

impl<'a> FileValue<'a> {
    #[cfg(test)]
    pub fn new(hash: &'a str, is_executable: bool) -> Self {
        Self {
            hash,
            is_executable,
        }
    }

    pub fn hash(&self) -> &str {
        self.hash
    }

    pub fn is_executable(&self) -> bool {
        self.is_executable
    }
}
