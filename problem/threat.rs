//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    Problem,
    severity::Severity
};

//> HEAD -> ISSUE
use issue::Issue;

//> HEAD -> ARRAY
use array::Array;


//^
//^ THREAT
//^

//> THREAT -> STRUCT
pub struct Threat<Object: Into<Issue>, const N: usize> {
    pub object: Object,
    pub chain: Array<&'static str, N>,
    pub severity: Severity
}

//> THREAT -> TRAIT
pub const trait Threaten {
    fn convert<Object: Into<Issue>, const N: usize>(&mut self, threat: Threat<Object, N>) -> Problem;
}