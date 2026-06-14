//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::toissue::ToIssue;

//> HEAD -> STD
use std::time::Instant;


//^
//^ PROBLEM
//^

//> PROBLEM -> STRUCT
pub struct Problem<Object: ToIssue> {
    pub at: Instant,
    object: Object
}

//> PROBLEM -> IMPLEMENTATION
impl<Object: ToIssue> Problem<Object> {
    pub fn new(object: Object) -> Self {return Self {
        at: Instant::now(),
        object: object
    }}
}