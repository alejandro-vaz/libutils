//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> FEATURES
#![feature(const_trait_impl)]
#![feature(const_convert)]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod severity;

//> HEAD -> SEVERITY
pub use severity::Severity;

//> HEAD -> ISSUE
use libutils_issue::Issue;

//> HEAD -> ALLOC
use alloc::vec::Vec;


//^
//^ THREAT
//^

//> THREAT -> STRUCT
pub struct Threat<Object: Into<Issue>> {
    pub object: Object,
    pub chain: Vec<&'static str>,
    pub severity: Severity
}