//^
//^ HEAD
//^

//> HEAD -> STD
use std::{
    fs::File,
    io::{
        Read,
        Write
    }
};

//> HEAD -> SUPER
use super::{
    metadata::Metadata,
    ioerror::IoError
};

//> HEAD -> CORE
use core::mem::transmute_neo as transmute;


//^
//^ DESCRIPTOR
//^

//> DESCRIPTOR -> STRUCT
pub struct Descriptor {
    pub file: File
}

//> DESCRIPTOR -> DESCRIPTOR
impl Descriptor {
    pub fn metadata(&self) -> Result<Metadata, IoError> {return match self.file.metadata() {
        Ok(metadata) => Ok(unsafe {transmute::<_, Metadata>(metadata)}),
        Err(error) => Err(IoError::CouldntReadMetadata {error})
    }}
    pub fn read_bytes(&mut self) -> Result<Vec<u8>, IoError> {
        let mut buffer = Vec::with_capacity(self.metadata()?.size());
        match self.file.read_to_end(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(error) => Err(IoError::CouldntReadFile {error})
        }
    }
    pub fn read(&mut self) -> Result<String, IoError> {
        return String::from_utf8(self.read_bytes()?).map_err(|error| IoError::FailedToEncodeRead {error: error})
    }
    pub fn write_bytes(&mut self, content: &[u8]) -> Result<(), IoError> {return match self.file.write(content) {
        Ok(_) => Ok(()),
        Err(error) => Err(IoError::CouldntWriteToFile {error})
    }}
    pub fn write(&mut self, content: &str) -> Result<(), IoError> {return self.write_bytes(content.as_bytes())}
}