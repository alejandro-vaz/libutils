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
pub struct Act<Type, Object: ToIssue, const NAME: &'static str> {
    pub problems: Vec<Problem<Object>>,
    pub result: Option<Type>
}

//> ACT -> TERMINATION
impl<Object: ToIssue> Termination for Act<(), Object, "Main"> {
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
impl<Type, Object: ToIssue, const NAME: &'static str> Act<Type, Object, NAME> {
    #[inline]
    pub fn attach<'valid, const OTHER: &'static str>(self, report: &'valid mut Report<Object, OTHER>) -> Attachment<'valid, Type, Object, OTHER> {
        for mut problem in self.problems {
            problem.chain.push(OTHER);
            report.problems.push(problem);
        };
        report.problems.sort_by(|first, second| first.at.cmp(&second.at));
        return Attachment {
            report: Some(report),
            result: self.result
        }
    }
    #[inline]
    pub fn map<Return, Closure: FnOnce(Type) -> Return>(self, closure: Closure) -> Act<Return, Object, NAME> {return Act {
        problems: self.problems,
        result: self.result.map(closure)
    }}
}