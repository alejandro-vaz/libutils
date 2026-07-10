//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> FEATURES
#![feature(const_trait_impl)]
#![feature(const_destruct)]
#![feature(test)]

//> HEAD -> CRATES
extern crate alloc;
extern crate test;

//> HEAD -> MODULES
mod argument;
#[cfg(test)]
mod benches;
mod descriptor;
mod metadata;
#[cfg(test)]
mod tests;
mod update;

//> HEAD -> ARGUMENT
pub use argument::Argument;

//> HEAD -> ISSUING
use issuing::Issue;

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
    fn arguments() -> &'static [Argument];
    fn open(filename: &str) -> Result<impl Descriptor, Issue>;
    fn problem(issue: Issue, chain: &[&'static str]) -> impl Update;
    fn print(value: impl Display) -> impl Update;
    fn debug(value: impl Debug) -> impl Update;
    fn clear() -> impl Update;
}