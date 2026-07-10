//^
//^ HEAD
//^

//> HEAD -> ISSUE
use libutils_issue::{
    Issue,
    Severity
};

//> HEAD -> ALLOC
use alloc::{
    string::{
        String,
        ToString
    },
    vec::Vec
};

//> HEAD -> SUPER
use super::metadata::Metadata;


//^
//^ DESCRIPTOR
//^

//> DESCRIPTOR -> TRAIT
#[must_use]
pub trait Descriptor: Sized {
    fn metadata(&self) -> Result<impl Metadata, Issue>;
    fn read_bytes(&mut self) -> Result<Vec<u8>, Issue>;
    fn read(&mut self) -> Result<String, Issue> {return String::from_utf8(self.read_bytes()?).map_err(|error| Issue {
        name: "Error encoding file to UTF-8",
        description: Some(error.to_string()),
        severity: Severity::Error
    })}
    fn write_bytes(&mut self, content: &[u8]) -> Result<(), Issue>;
    fn write(&mut self, content: &str) -> Result<(), Issue> {return self.write_bytes(content.as_bytes())}
    fn close(self) -> () {}
}