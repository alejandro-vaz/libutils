//^
//^ HEAD
//^

//> HEAD -> MODULES
mod action;
mod issue;
mod toissue;

//> HEAD -> STD
use std::time::Instant;

//> HEAD -> CORE
use core::default::Default;

//> HEAD -> ACTION
pub use action::Action;

//> HEAD -> TOISSUE
pub use toissue::ToIssue;

//> HEAD -> ISSUE
pub use issue::Issue;


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
    pub fn attach<Inferior>(&mut self, action: Action<Problem, Inferior>) -> Option<Inferior> {
        self.problems.extend(action.problems);
        return action.value;
    }
}