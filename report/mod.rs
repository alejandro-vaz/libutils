//^
//^ HEAD
//^

//> HEAD -> MODULES
mod issue;
mod note;
mod shortcut;
mod toissue;

//> HEAD -> STD
use std::{
    process::{
        ExitCode, 
        Termination
    }, 
    time::Instant
};

//> HEAD -> TOISSUE
pub use toissue::ToIssue;

//> HEAD -> ISSUE
pub use issue::Issue;

//> HEAD -> CRATE
use crate::terminal::{
    TERMINAL,
    Problem
};

//> HEAD -> SHORTCUT
use shortcut::Attachment;

//> HEAD -> NOTE
pub use note::Note;


//^
//^ REPORT
//^

//> REPORT -> STRUCT
pub struct Report<Object: ToIssue> {
    name: &'static str = "",
    problems: Vec<Problem<Object>> = Vec::new()
}

//> REPORT -> IMPLEMENTATION
impl<Object: ToIssue> Report<Object> {
    #[inline]
    pub fn new(name: &'static str) -> Self {return Self {
        name: name,
        ..
    }}
    #[inline]
    pub fn warn(&mut self, object: Object) -> () {
        let problem = Problem {
            chain: Vec::from([self.name]),
            at: Instant::now(),
            object: object
        };
        TERMINAL.write().error(&problem);
        self.problems.push(problem);
    }
    #[inline]
    pub unsafe fn abort<Type>(self) -> Act<Type, Object> {return Act {
        problems: self.problems,
        result: None
    }}
    #[inline]
    pub fn fail<Type>(mut self, with: Object) -> Act<Type, Object> {
        let problem = Problem {
            chain: Vec::from([self.name]),
            at: Instant::now(),
            object: with
        };
        TERMINAL.write().error(&problem);
        self.problems.push(problem);
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
    //#[inline]
    //pub fn take<'valid, Inferior>(&'valid mut self, option: Option<Inferior>, object: Object) -> Attachment<'valid, Inferior, Object> {
    //    if option.is_none() {self.warn(object)}
    //    return Attachment {
    //        report: Some(self),
    //        result: option
    //    };
    //}
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

//> ACT -> ATTACH
impl<Type, Object: ToIssue> Act<Type, Object> {
    #[inline]
    pub fn attach<'valid>(self, report: &'valid mut Report<Object>) -> Attachment<'valid, Type, Object> {
        for mut problem in self.problems {
            problem.chain.push(report.name);
            report.problems.push(problem);
        };
        report.problems.sort_by(|first, second| first.at.cmp(&second.at));
        return Attachment {
            report: Some(report),
            result: self.result
        }
    }
}
