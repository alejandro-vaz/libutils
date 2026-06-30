//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> FEATURES
#![feature(const_convert)]
#![feature(const_trait_impl)]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod conversions;
mod severity;
#[cfg(test)]
mod tests;

//> HEAD -> ALLOC
use alloc::string::String;

//> HEAD -> SEVERITY
pub use severity::Severity;


//^
//^ ISSUE
//^

//> ISSUE -> STRUCT
#[derive(Hash)]
pub struct Issue {
    pub name: &'static str,
    pub description: Option<String>,
    pub severity: Severity
}