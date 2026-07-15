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
#![feature(final_associated_functions)]
#![feature(default_field_values)]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod argument;
mod clitype;
mod descriptor;
mod metadata;
mod update;

//> HEAD -> ARGUMENT
pub use argument::Argument;

//> HEAD -> ISSUING
use issuing::Issue;

//> HEAD -> CORE
use core::fmt::{
    Debug, 
    Display
};

//> HEAD -> UPDATE
pub use update::Update;

//> HEAD -> METADATA
pub use metadata::Metadata;

//> HEAD -> DESCRIPTOR
pub use descriptor::Descriptor;

//> HEAD -> CLITYPE
pub use clitype::CliType;


//^
//^ SYSTEMIO
//^

//> SYSTEMIO -> TRAIT
pub trait SystemIO {
    fn arguments() -> &'static [Argument];
    fn open(filename: &str) -> Result<impl Descriptor, Issue>;
    fn warning(error: impl Into<Issue>, chain: &[&'static str]) -> impl Update;
    fn error(error: impl Into<Issue>, chain: &[&'static str]) -> impl Update;
    fn critical(error: impl Into<Issue>, chain: &[&'static str]) -> !;
    final fn expect<Type>(result: Result<Type, impl Into<Issue>>, chain: &[&'static str]) -> Type {
        return match result {
            Ok(value) => value,
            Err(error) => Self::critical(error, chain)
        }
    }
    fn print(value: impl Display) -> impl Update;
    fn debug(value: impl Debug) -> impl Update;
    fn clear() -> impl Update;
}