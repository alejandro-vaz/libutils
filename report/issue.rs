//^
//^ HEAD
//^

//> HEAD -> CORE
use core::hash::Hash;


//^
//^ ISSUE
//^

//> ISSUE -> STRUCT
#[derive(Clone, Hash)]
pub struct Issue {
    pub name: &'static str,
    pub description: Option<String>
}