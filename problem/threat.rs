//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::{
    issue::Issue,
    array::Array
};

//> HEAD -> SUPER
use super::{
    Problem,
    severity::Severity
};


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