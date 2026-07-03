//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod argument;
mod handle;

//> HEAD -> THREAT
use libutils_threat::Threat;

//> HEAD -> ARGUMENT
pub use argument::Argument;

//> HEAD -> ISSUE
use libutils_issue::Issue;

//> HEAD -> ALLOC
use alloc::string::String;

//> HEAD -> CORE
use core::fmt::{
    Display,
    Debug
};

//> HEAD -> HANDLE
pub use handle::Handle;


//^
//^ CONSOLE
//^

//> CONSOLE -> TRAIT
pub trait Console {
    fn arguments<'valid>(&'valid self) -> &'valid [Argument];
    fn read(&self, filename: &str) -> Result<String, Issue>;
    fn problem(&self, threat: Threat) -> impl Handle;
    fn print<Type: Display>(&self, value: Type) -> impl Handle;
    fn debug<Type: Debug>(&self, value: Type) -> impl Handle;
}