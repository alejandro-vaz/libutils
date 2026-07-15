//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> FEATURES
#![feature(const_convert)]
#![feature(default_field_values)]
#![feature(const_trait_impl)]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod conversions;

//> HEAD -> ALLOC
use alloc::string::String;

//> HEAD -> CORE
use core::hash::{
    Hash,
    Hasher
};


//^
//^ ISSUE
//^

//> ISSUE -> STRUCT
#[derive(Debug)]
pub struct Issue {
    pub name: &'static str,
    pub description: Option<String> = None
}

//> ISSUE -> HASH
impl Hash for Issue {
    fn hash<H: Hasher>(&self, state: &mut H) {Hash::hash(self.name, state)}
}