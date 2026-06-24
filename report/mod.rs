//^
//^ HEAD
//^

//> HEAD -> MODULES
mod act;
mod issue;
mod note;
mod shortcut;
mod toissue;

//> HEAD -> STD
use std::time::Instant;

//> HEAD -> TOISSUE
pub use toissue::ToIssue;

//> HEAD -> ISSUE
pub use issue::Issue;

//> HEAD -> CRATE
use crate::terminal::{
    TERMINAL,
    Problem,
    Console,
    Severity
};

//> HEAD -> NOTE
pub use note::Note;

//> HEAD -> ACT
pub use act::Act;


//^
//^ REPORT
//^

//> REPORT -> STRUCT
#[derive(Default)]
pub struct Report<Object: ToIssue, const NAME: &'static str> {
    problems: Vec<Problem<Object>> = Vec::new()
}

//> REPORT -> IMPLEMENTATION
impl<Object: ToIssue, const NAME: &'static str> Report<Object, NAME> {
    #[inline]
    pub fn warn(&mut self, object: Object) -> () {
        let problem = Problem {
            chain: Vec::from([NAME]),
            at: Instant::now(),
            object: object,
            severity: Severity::Warning
        };
        TERMINAL.write().issue(&problem);
        self.problems.push(problem);
    }
    #[inline]
    pub fn error(&mut self, object: Object) -> () {
        let problem = Problem {
            chain: Vec::from([NAME]),
            at: Instant::now(),
            object: object,
            severity: Severity::Error
        };
        TERMINAL.write().issue(&problem);
        self.problems.push(problem);
    }
    #[inline]
    pub fn finish<Type>(mut self, with: Result<Type, Object>) -> Act<Type, Object, NAME> {return match with {
        Ok(value) => Act {
            problems: self.problems,
            result: Some(value)
        },
        Err(object) => {
            let problem = Problem {
                chain: Vec::from([NAME]),
                at: Instant::now(),
                object: object,
                severity: Severity::Critical
            };
            TERMINAL.write().issue(&problem);
            self.problems.push(problem);
            Act {
                problems: self.problems,
                result: None
            }
        }
    }}
}