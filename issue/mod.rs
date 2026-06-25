//^
//^ HEAD
//^

//> HEAD -> MODULES
mod conversions;

//> HEAD -> CORE
use core::hash::Hash;


//^
//^ ISSUE
//^

//> ISSUE -> STRUCT
#[derive(Hash)]
pub struct Issue {
    pub name: &'static str,
    pub description: Option<String>
}