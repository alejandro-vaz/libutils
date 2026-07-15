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
use core::{
    hash::{
        Hash,
        Hasher
    },
    any::TypeId
};


//^
//^ ISSUE
//^

//> ISSUE -> STRUCT
#[derive(Debug, PartialEq, Eq)]
pub struct Issue {
    pub name: &'static str,
    pub id: Option<TypeId> = None,
    pub description: Option<String> = None
}

//> ISSUE -> HASH
impl Hash for Issue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        return Hash::hash(&(self.name, self.id), state);
    }
}