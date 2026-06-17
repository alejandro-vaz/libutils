//^
//^ HEAD
//^

//> HEAD -> MODULES
mod issue;
mod problem;
mod shortcut;
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

//> HEAD -> SHORTCUT
use shortcut::Attachment;


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
        TERMINAL.write().error(object.to_issue());
        self.problems.push(Problem::new(object));
    }
    #[inline]
    pub unsafe fn abort<Type>(self) -> Act<Type, Object> {return Act {
        problems: self.problems,
        result: None
    }}
    #[inline]
    pub fn fail<Type>(mut self, with: Object) -> Act<Type, Object> {
        TERMINAL.write().error(with.to_issue());
        self.problems.push(Problem::new(with));
        return Act {
            problems: self.problems,
            result: None
        }
    }
    #[inline]
    pub fn succeed<Type>(self, value: Type) -> Act<Type, Object> {return Act {
        problems: self.problems,
        result: Some(value)
    }}
    #[inline]
    pub fn attach<'valid, Inferior>(&'valid mut self, act: Act<Inferior, Object>) -> Attachment<'valid, Inferior, Object> {
        self.problems.extend(act.problems);
        self.problems.sort_by(|first, second| first.at.cmp(&second.at));
        return Attachment {
            report: Some(self),
            result: act.result
        }
    }
}


//^
//^ ACT
//^

//> ACT -> STRUCT
pub struct Act<Type, Object: ToIssue> {
    problems: Vec<Problem<Object>>,
    pub result: Option<Type>
}

//> ACT -> TERMINATION
impl<Object: ToIssue> Termination for Act<(), Object> {
    fn report(self) -> ExitCode {return match self.result {
        Some(()) => ExitCode::SUCCESS,
        None => self.problems.last().unwrap().object.to_issue().code
    }}
}