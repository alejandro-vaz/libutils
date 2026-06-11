//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod action;
pub mod issue;
pub mod toissue;

//> HEAD -> STD
use std::time::Instant;

//> HEAD -> CORE
use core::default::Default;

//> HEAD -> ACTION
use action::Action;

//> HEAD -> TOISSUE
use toissue::ToIssue;


//^
//^ REPORT
//^

//> REPORT -> STRUCT
pub struct Report<Problem: ToIssue> {
    start: Instant,
    problems: Vec<Problem>
}

//> REPORT -> DEFAULT
impl<Problem: ToIssue> Default for Report<Problem> {
    fn default() -> Self {return Self {
        start: Instant::now(),
        problems: Vec::new()
    }}
}

//> REPORT -> IMPLEMENTATION
impl<Problem: ToIssue> Report<Problem> {
    #[inline]
    pub fn warn(&mut self, problem: Problem) -> () {self.problems.push(problem)}
    pub fn fail<Type>(self) -> Action<Problem, Type> {return Action {
        start: self.start,
        duration: Instant::duration_since(&Instant::now(), self.start),
        problems: self.problems,
        value: None
    }}
    pub fn conclude<Type>(self, value: Type) -> Action<Problem, Type> {return Action {
        start: self.start,
        duration: Instant::duration_since(&Instant::now(), self.start),
        problems: self.problems,
        value: Some(value)
    }}
}