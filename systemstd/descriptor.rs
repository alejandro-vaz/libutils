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

//> HEAD -> SYSTEMIO
use systemio::{
    Metadata as MetadataTrait,
    Descriptor as DescriptorTrait
};

//> HEAD -> ISSUING
use issuing::Issue;

//> HEAD -> SUPER
use super::metadata::Metadata;

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
impl DescriptorTrait for Descriptor {
    fn metadata(&self) -> Result<impl MetadataTrait, Issue> {return match self.file.metadata() {
        Ok(metadata) => Ok(unsafe {transmute::<_, Metadata>(metadata)}),
        Err(error) => Err(Issue {
            name: "Failed to read size of file",
            description: Some(error.to_string()),
            ..
        })
    }}
    fn read_bytes(&mut self) -> Result<Vec<u8>, Issue> {
        let mut buffer = Vec::with_capacity(self.metadata()?.size());
        match self.file.read_to_end(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(error) => Err(Issue {
                name: "Failed to read file as binary",
                description: Some(error.to_string()),
                ..
            })
        }
    }
    fn write_bytes(&mut self, content: &[u8]) -> Result<(), Issue> {return match self.file.write(content) {
        Ok(_) => Ok(()),
        Err(error) => Err(Issue {
            name: "Failure writing to file",
            description: Some(error.to_string()),
            ..
        })
    }}
}