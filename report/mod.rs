//^
//^ HEAD
//^

//> HEAD -> MODULES
mod issue;
mod problem;
mod toissue;

//> HEAD -> STD
use std::process::{
    Termination,
    ExitCode
};

//> HEAD -> CORE
use core::default::Default;

//> HEAD -> TOISSUE
pub use toissue::ToIssue;

//> HEAD -> ISSUE
pub use issue::Issue;

//> HEAD -> PROBLEM
use problem::Problem;

//> HEAD -> CRATE
use crate::terminal::TERMINAL;


//^
//^ REPORT
//^

//> REPORT -> STRUCT
pub struct Report<Object: ToIssue> {
    problems: Vec<Problem<Object>>
}

//> REPORT -> DEFAULT
impl<Object: ToIssue> const Default for Report<Object> {
    fn default() -> Self {return Self {
        problems: Vec::new()
    }}
}

//> REPORT -> IMPLEMENTATION
impl<Object: ToIssue> Report<Object> {
    #[inline]
    pub fn warn(&mut self, object: Object) -> () {
        TERMINAL.free().error(object.to_issue());
        self.problems.push(Problem::new(object));
    }
    #[inline]
    pub fn fail<Type>(self) -> Act<Type, Object> {return Act {
        problems: self.problems,
        result: None
    }}
    #[inline]
    pub fn succeed<Type>(self, value: Type) -> Act<Type, Object> {return Act {
        problems: self.problems,
        result: Some(value)
    }}
    #[inline]
    pub fn attach<Inferior>(&mut self, act: Act<Inferior, Object>) -> Option<Inferior> {
        self.problems.extend(act.problems);
        self.problems.sort_by(|first, second| first.at.cmp(&second.at));
        return act.result;
    }
}


//^
//^ ACT
//^

//> ACT -> STRUCT
pub struct Act<Type, Object: ToIssue> {
    problems: Vec<Problem<Object>>,
    result: Option<Type>
}

//> ACT -> TERMINATION
impl<Object: ToIssue> Termination for Act<(), Object> {
    fn report(self) -> ExitCode {
        return match self.result {
            Some(()) => ExitCode::SUCCESS,
            None => if let Some(problem) = self.problems.last() {
                problem.object.to_issue().code
            } else {ExitCode::from(255)}
        }
    }
}

//> ACT -> IMPLEMENTATION
impl<Type, Object: ToIssue> Act<Type, Object> {
    pub fn result(self) -> Option<Type> {return self.result}
}