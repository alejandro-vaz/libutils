//^
//^ HEAD
//^

//> HEAD -> MODULES
mod action;
mod issue;
mod problem;
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

//> HEAD -> PROBLEM
use problem::Problem;


//^
//^ REPORT
//^

//> REPORT -> STRUCT
pub struct Report<Object: ToIssue, Output: Fn(String) -> ()> {
    start: Instant,
    problems: Vec<Problem<Object>>,
    output: Output
}

//> REPORT -> DEFAULT
impl<Object: ToIssue> Default for Report<Object, fn(String) -> ()> {
    fn default() -> Self {return Self {
        start: Instant::now(),
        problems: Vec::new(),
        output: |string| eprintln!("{string}")
    }}
}

//> REPORT -> IMPLEMENTATION
impl<Object: ToIssue, Output: Fn(String) -> ()> Report<Object, Output> {
    pub fn buffered(output: Output) -> Self {return Self {
        start: Instant::now(),
        problems: Vec::new(),
        output: output
    }}
    #[inline]
    pub fn warn(&mut self, object: Object) -> () {
        (self.output)(object.to_issue().to_string());
        self.problems.push(Problem::new(object));
    }
    pub fn abort<Type>(self) -> Action<Type, Object> {return Action {
        start: self.start,
        duration: Instant::duration_since(&Instant::now(), self.start),
        problems: self.problems,
        value: None
    }}
    pub fn conclude<Type>(self, value: Type) -> Action<Type, Object> {return Action {
        start: self.start,
        duration: Instant::duration_since(&Instant::now(), self.start),
        problems: self.problems,
        value: Some(value)
    }}
    pub fn attach<Inferior>(&mut self, action: Action<Inferior, Object>) -> Option<Inferior> {
        self.problems.extend(action.problems);
        self.problems.sort_by(|first, second| first.at.cmp(&second.at));
        return action.value;
    }
}