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
#[derive(Debug, PartialEq, Eq)]
pub struct Issue {
    pub name: &'static str,
    pub description: Option<String> = None
}

//> ISSUE -> IMPLEMENTATION
impl Issue {
    pub fn assert_normal(&self) -> () {for (condition, message) in [
        (self.name_startlowercase(), "name doesn't start with lowercase"),
        (self.name_notempty(), "name is empty"),
        (self.name_nopadding(), "name has whitespace padding"),
        (self.name_nodots(), "name has dot (.) characters"),
        (self.name_nodoublespace(), "name has double spaces in it"),
        (self.description_notempty(), "description is empty or none")
    ] {assert!(condition, "{message}: {self:#?}")}}
    fn name_startlowercase(&self) -> bool {
        return self.name.chars().next().map(|character| character.is_lowercase()).unwrap_or_default();
    }
    fn name_notempty(&self) -> bool {return !self.name.is_empty()}
    fn name_nopadding(&self) -> bool {return self.name.trim() == self.name}
    fn name_nodots(&self) -> bool {self.name.chars().all(|character| character != '.')}
    fn name_nodoublespace(&self) -> bool {
        let mut before = false;
        for character in self.name.chars() {
            if character.is_whitespace() {
                if before {return false} else {before = true;}
            } else {before = false}
        }
        return true;
    }
    fn description_notempty(&self) -> bool {return if let Some(string) = &self.description && !string.is_empty() {true} else {false}}
}

//> ISSUE -> HASH
impl Hash for Issue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        return Hash::hash(&self.name, state);
    }
}
