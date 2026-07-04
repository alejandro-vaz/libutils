//^
//^ HEAD
//^

//> HEAD -> ALLOC
use alloc::{
    string::String,
    vec::Vec
};

//> HEAD -> ISSUE
use libutils_issue::Issue;


//^
//^ CONTINUATIONS
//^

//> CONTINUATIONS -> TRAIT
#[must_use]
pub trait Synchronization {
    fn sync(self) -> ();
    fn ignore(self) -> ();
}

//> CONTINUATIONS -> DESCRIPTOR
#[must_use]
pub trait Descriptor {
    fn read(&mut self) -> Result<String, Issue>;
    fn read_bytes(&mut self) -> Result<Vec<u8>, Issue>;
    fn close(self) -> ();
    fn write(&mut self, content: &str) -> Result<(), Issue>;
    fn write_bytes(&mut self, content: &[u8]) -> Result<(), Issue>;
}