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
    pub description: Option<String> = None,
    pub help: Option<String> = None,
    pub traceback: Option<String> = None
}

//> ISSUE -> IMPLEMENTATION
impl Issue {
    pub fn assert_normal(self) -> Self {
        for (condition, message) in [
            (!self.name.is_empty(), "name is empty"),
            (
                self.name.chars().next().map(|c| c.is_lowercase()).unwrap_or_default(), 
                "name doesn't start with lowercase"
            ),
            (self.name.trim().len() == self.name.len(), "name has whitespace padding"),
            (self.name.chars().all(|c| c != '.'), "name has dot (.) characters"),
            ((|| {
                let mut before = false;
                for character in self.name.chars() {
                    if character.is_whitespace() {
                        if before {return false} else {before = true}
                    } else {before = false}
                }
                true
            })(), "name has double spaces in it"),
            (!matches!(
                self.description.as_ref().map(String::as_str), 
                Some("")
            ), "description is empty or none")
        ] {debug_assert!(condition, "{message}: {self:#?}")};
        return self;
    }
}

//> ISSUE -> HASH
impl Hash for Issue {
    fn hash<H: Hasher>(&self, state: &mut H) {return Hash::hash(&self.name, state)}
}
