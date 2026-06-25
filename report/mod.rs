//^
//^ HEAD
//^

//> HEAD -> MODULES
mod act;
mod machine;
mod shortcut;

//> HEAD -> STD
use std::time::Instant;

//> HEAD -> CRATE
use crate::{
    problem::{
        Problem,
        Severity
    },
    issue::Issue
};

//> HEAD -> ACT
pub use act::Act;

//> HEAD -> MACHINE
pub use machine::{
    Main,
    Mode,
    Set
};


//^
//^ REPORT
//^

//> REPORT -> STRUCT
pub struct Report<State: Mode>(State);

//> REPORT -> DEFAULT
impl Default for Report<Main> {
    fn default() -> Self {return Self(Main)}
}

//> REPORT -> IMPLEMENTATION
impl<State: Mode> Report<State> {
    #[inline]
    pub fn warn<Object: Into<Issue>>(&self, object: Object) -> () {
        self.0.send(Problem {
            chain: Vec::new(),
            at: Instant::now(),
            object: object,
            severity: Severity::Warning
        });
    }
    #[inline]
    pub fn error<Object: Into<Issue>>(&self, object: Object) -> () {
        self.0.send(Problem {
            chain: Vec::new(),
            at: Instant::now(),
            object: object,
            severity: Severity::Error
        });
    }
    #[inline]
    pub fn sub<'valid, const NAME: &'static str>(&'valid self) -> Report<Set<'valid, NAME>> {return Report(self.0.connect())}
    #[inline]
    pub fn with<Type>(self, value: Type) -> Act<Type, {State::NAME}> {return Act(Some(value))}
    #[inline]
    pub fn with_default<Type: Default>(self) -> Act<Type, {State::NAME}> {return Act(Some(Type::default()))}
    #[inline]
    pub fn fail<Type, Object: Into<Issue>>(self, object: Object) -> Act<Type, {State::NAME}> {
        self.0.send(Problem {
            chain: Vec::new(),
            at: Instant::now(),
            object: object,
            severity: Severity::Critical
        });
        return Act(None);
    }
}