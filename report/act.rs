//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    toissue::ToIssue,
    Report,
    shortcut::Attachment
};

//> HEAD -> CRATE
use crate::terminal::Problem;

//> HEAD -> CORE
use core::hash::{
    Hash,
    Hasher
};

//> HEAD -> STD
use std::{
    hash::DefaultHasher,
    process::{
        ExitCode,
        Termination
    }
};


//^
//^ ACT
//^

//> ACT -> STRUCT
pub struct Act<Type, Object: ToIssue> {
    pub problems: Vec<Problem<Object>>,
    pub result: Option<Type>
}

//> ACT -> TERMINATION
impl<Object: ToIssue> Termination for Act<(), Object> {
    fn report(self) -> ExitCode {return match self.result {
        Some(()) => ExitCode::SUCCESS,
        None => match self.problems.last() {
            None => unreachable!(),
            Some(problem) => {
                let mut state = DefaultHasher::new();
                problem.object.to_issue().hash(&mut state);
                ExitCode::from(state.finish() as u8)
            }
        }
    }}
}

//> ACT -> IMPLEMENTATION
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
    #[inline]
    pub fn map<Return, Closure: FnOnce(Type) -> Return>(self, closure: Closure) -> Act<Return, Object> {return Act {
        problems: self.problems,
        result: self.result.map(closure)
    }}
}