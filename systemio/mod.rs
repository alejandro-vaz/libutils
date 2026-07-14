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
#![feature(default_field_values)]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod argument;
mod descriptor;
mod metadata;
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
//^ SYSTEMIO
//^

//> SYSTEMIO -> TRAIT
pub trait SystemIO {
    fn arguments() -> &'static [Argument];
    fn open(filename: &str) -> Result<impl Descriptor, Issue>;
    fn problem(issue: Issue, chain: &[&'static str]) -> impl Update;
    fn print(value: impl Display) -> impl Update;
    fn debug(value: impl Debug) -> impl Update;
    fn clear() -> impl Update;
}