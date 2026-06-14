//^
//^ HEAD
//^

//> HEAD -> STD
use std::time::Instant;

//> HEAD -> CORE
use core::time::Duration;

//> HEAD -> SUPER
use super::{
    toissue::ToIssue,
    problem::Problem
};


//^
//^ ACTION
//^

//> ACTION -> STRUCT
pub struct Action<Type, Object: ToIssue> {
    pub start: Instant,
    pub duration: Duration,
    pub problems: Vec<Problem<Object>>,
    pub value: Option<Type>
}