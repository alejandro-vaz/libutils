//^
//^ HEAD
//^

//> HEAD -> STD
use std::{
    fs::File as StdFile,
    io::Read
};

//> HEAD -> SUPER
use super::{
    metadata::Metadata,
    ioerror::IoError
};

//> HEAD -> CORE
use core::io::Write;


//^
//^ DESCRIPTOR
//^

//> DESCRIPTOR -> STRUCT
pub struct Descriptor<const WRITE: bool> {
    pub stdfile: StdFile
}

//> DESCRIPTOR -> IMPLEMENTATION
impl<const WRITE: bool> Descriptor<WRITE> {
    //pub fn lock(&self) -> () {}
    // try_lock
    // try_shared_lock
    // shared_lock
    pub fn metadata(&self) -> Result<Metadata, IoError<'static>> {
        return match self.stdfile.metadata() {
            Ok(metadata) => Ok(Metadata {
                metadata: metadata
            }),
            Err(error) => Err(IoError::CouldntReadMetadata {error})
        }
    }
    pub fn read_bytes(&mut self) -> Result<Vec<u8>, IoError<'static>> {
        let mut buffer = Vec::with_capacity(self.metadata()?.size());
        match self.stdfile.read_to_end(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(error) => Err(IoError::CouldntReadFile {error})
        }
    }
    pub fn read(&mut self) -> Result<String, IoError<'static>> {
        return String::from_utf8(self.read_bytes()?).map_err(|error| {
            IoError::FailedToEncodeRead {error: error}
        });
    }
}

//> DESCRIPTOR -> WRITE IMPLEMENTATION
impl Descriptor<true> {
    pub fn write_bytes(&mut self, content: &[u8]) -> Result<(), IoError<'static>> {
        return match self.stdfile.write(content) {
            Ok(_) => Ok(()),
            Err(error) => Err(IoError::CouldntWriteToFile {error})
        }
    }
    pub fn write(&mut self, content: &str) -> Result<(), IoError<'static>> {
        return self.write_bytes(content.as_bytes())
    }
}