//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> ISSUE
use libutils_issue::Issue;

//> HEAD -> ALLOC
use alloc::vec::Vec;


//^
//^ THREAT
//^

//> THREAT -> STRUCT
pub struct Threat {
    pub issue: Issue,
    pub chain: Vec<&'static str>
}