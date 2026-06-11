//^
//^ HEAD
//^

//> HEAD -> STD
use std::time::Instant;

//> HEAD -> CORE
use core::{
    fmt::{
        Display,
        Formatter,
        Result as Format
    },
    time::Duration
};

//> HEAD -> SUPER
use super::toissue::ToIssue;


//^
//^ ACTION
//^

//> ACTION -> STRUCT
pub struct Action<Problem: ToIssue, Type> {
    pub start: Instant,
    pub duration: Duration,
    pub(super) problems: Vec<Problem>,
    pub(super) value: Option<Type>
}

//> ACTION -> DISPLAY
impl<Problem: ToIssue, Type> Display for Action<Problem, Type> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {
        for problem in self.problems.iter() {write!(formatter, "{}", problem.to_issue())?};
        return Ok(());
    }
}

//> ACTION -> IMPLEMENTATION
impl<Problem: ToIssue, Type> Action<Problem, Type> {
    pub fn problems<'valid>(&'valid self) -> &'valid [Problem] {return &self.problems}
    #[inline]
    pub fn result(self) -> Option<Type> {return self.value}
}