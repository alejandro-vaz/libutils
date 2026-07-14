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
#![feature(transmute_neo)]
#![feature(never_type)]

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
use issuing::{
    Issue,
    Severity
};

//> HEAD -> CORE
use core::fmt::{
    Display,
    Debug
};
use core::mem::transmute_neo as transmute;
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
    fn problem(error: impl Into<Issue>, chain: &[&'static str]) -> impl Update;
    final fn expect<Type>(result: Result<Type, impl Into<Issue>>, chain: &[&'static str]) -> Type {return match result {
        Ok(value) => value,
        Err(error) => {
            let mut issue = Into::<Issue>::into(error);
            issue.severity = Severity::Critical;
            unsafe {transmute::<_, !>(Self::problem(issue, chain))};
        }
    }}
    fn print(value: impl Display) -> impl Update;
    fn debug(value: impl Debug) -> impl Update;
    fn clear() -> impl Update;
}