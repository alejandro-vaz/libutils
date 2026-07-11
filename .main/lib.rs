//^
//^ LIBUTILS
//^

//> HEAD -> NO STD
#![no_std]

//> LIBUTILS -> DOCS
#![doc = include_str!("README.md")]

//> LIBUTILS -> IMPORTS
#[cfg(feature = "no-alloc")]
pub use noalloc::*;
#[cfg(feature = "no-std")] 
pub use nostd::*;
#[cfg(feature = "std")] 
pub use std::*;

//> LIBUTILS -> NO ALLOC
#[cfg(feature = "no-alloc")] 
mod noalloc {
    pub extern crate constrangeiter;
}

//> LIBUTILS -> NO STD
#[cfg(feature = "no-std")] 
mod nostd {
    pub extern crate bytediff;
    pub extern crate ebnftobnf;
    pub extern crate issuing;
    pub extern crate stack_array;
    pub extern crate systemio;
}

//> LIBUTILS -> STD
#[cfg(feature = "std")] 
mod std {
    pub extern crate active_reporting;
    pub extern crate cagelock;
    pub extern crate systemstd;
}