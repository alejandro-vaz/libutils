//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod argument;

//> HEAD -> PROBLEM
use libutils_threat::Threat;

//> HEAD -> ARGUMENT
pub use argument::Argument;

//> HEAD -> ISSUE
use libutils_issue::Issue;

//> HEAD -> ALLOC
use alloc::string::String;


//^
//^ CONSOLE
//^

//> CONSOLE -> TRAIT
pub trait Console {
    fn arguments(&self) -> &[Argument];
    fn read(&self, filename: &str) -> Result<String, Issue>;
    fn sync(&mut self) -> ();
    fn problem(&mut self, threat: Threat) -> ();
}