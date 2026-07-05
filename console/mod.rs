//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> FEATURES
#![feature(const_trait_impl)]
#![feature(const_destruct)]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod argument;
mod descriptor;
mod metadata;
mod update;

//> HEAD -> ARGUMENT
pub use argument::Argument;

//> HEAD -> ISSUE
use libutils_issue::Issue;

//> HEAD -> CORE
use core::fmt::{
    Display,
    Debug
};

//> HEAD -> UPDATE
pub use update::Update;

//> HEAD -> METADATA
pub use metadata::Metadata;

//> HEAD -> DESCRIPTOR
pub use descriptor::Descriptor;


//^
//^ CONSOLE
//^

//> CONSOLE -> TRAIT
pub trait Console {
    fn arguments<'valid>(&'valid self) -> &'valid [Argument];
    fn open(&self, filename: &str) -> Result<impl Descriptor, Issue>;
    fn problem(&self, issue: Issue, chain: &[&'static str]) -> impl Update;
    fn print(&self, value: impl Display) -> impl Update;
    fn debug(&self, value: impl Debug) -> impl Update;
}