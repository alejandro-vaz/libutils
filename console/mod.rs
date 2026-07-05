//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod argument;
mod continuations;

//> HEAD -> ARGUMENT
pub use argument::Argument;

//> HEAD -> ISSUE
use libutils_issue::Issue;

//> HEAD -> CORE
use core::fmt::{
    Display,
    Debug
};

//> HEAD -> CONTINUATIONS
pub use continuations::{
    Synchronization,
    Descriptor,
    Data
};


//^
//^ CONSOLE
//^

//> CONSOLE -> TRAIT
pub trait Console {
    fn arguments<'valid>(&'valid self) -> &'valid [Argument];
    fn open(&self, filename: &str) -> Result<impl Descriptor, Issue>;
    fn problem(&self, issue: Issue, chain: &[&'static str]) -> impl Synchronization;
    fn print(&self, value: impl Display) -> impl Synchronization;
    fn debug(&self, value: impl Debug) -> impl Synchronization;
}