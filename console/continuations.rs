//^
//^ HEAD
//^

//> HEAD -> ALLOC
use alloc::{
    string::{
        String, 
        ToString
    }, 
    vec::Vec
};

//> HEAD -> ISSUE
use libutils_issue::{
    Issue,
    Severity
};


//^
//^ CONTINUATIONS
//^

//> CONTINUATIONS -> SYNCHRONIZATION
#[must_use]
pub trait Synchronization: Sized {
    fn sync(self) -> ();
    fn ignore(self) -> () {}
}

//> CONTINUATIONS -> DESCRIPTOR
#[must_use]
pub trait Descriptor: Sized {
    fn metadata(&self) -> Result<impl Data, Issue>;
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

//> CONTINUATIONS -> DATA
#[must_use]
pub trait Data: Sized {
    fn size(&self) -> usize;
}