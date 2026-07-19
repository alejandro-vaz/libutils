//^
//^ HEAD
//^

//> HEAD -> NO STD
#![no_std]

//> HEAD -> MODULES
mod dump;

//> HEAD -> ISSUING
use issuing::Issue;

//> HEAD -> DUMP
pub use dump::Dump;


//^
//^ SYSTEMIO
//^

//> SYSTEMIO -> STRUCT
pub struct SystemIO<Object: Into<Issue>> {
    pub warning: fn(Object, &[&'static str]) -> (),
    pub error: fn(Object, &[&'static str]) -> (),
    pub critical: fn(Object, &[&'static str]) -> !
}