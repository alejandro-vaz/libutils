//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::toissue::ToIssue;

//> HEAD -> STD
use std::time::Instant;

//> HEAD -> CORE
use core::fmt::{
    Debug,
    Formatter,
    Result as Format
};


//^
//^ PROBLEM
//^

//> PROBLEM -> STRUCT
pub struct Problem<Object: ToIssue> {
    pub at: Instant,
    pub object: Object
}

//> PROBLEM -> IMPLEMENTATION
impl<Object: ToIssue> Problem<Object> {
    pub fn new(object: Object) -> Self {return Self {
        at: Instant::now(),
        object: object
    }}
}

//> PROBLEM -> DEBUG
impl<Object: ToIssue> Debug for Problem<Object> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {write!(formatter, "Problem")}
}