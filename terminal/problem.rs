//^
//^ HEAD
//^

//> HEAD -> crate
use crate::report::{
    ToIssue,
    Issue
};

//> HEAD -> STD
use std::time::Instant;

//> HEAD -> CORE
use core::fmt::{
    Display,
    Formatter,
    Result as Format,
    Debug
};


//^
//^ PROBLEM
//^

//> PROBLEM -> STRUCT
pub struct Problem<Object: ToIssue> {
    pub chain: Vec<&'static str>,
    pub at: Instant,
    pub object: Object,
    pub severity: Severity
}

//> PROBLEM -> SEVERITY
#[derive(Clone, Copy, Debug)]
pub enum Severity {
    Warning,
    Error,
    Critical
}

//> PROBLEM -> DISPLAY
impl Display for Problem<Issue> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {write!(
        formatter,
        "@{}\n{:?}: {}{}",
        self.chain.iter().rev().map(|node| *node).collect::<Vec<&'static str>>().join(" > "),
        self.severity,
        self.object.name,
        if let Some(description) = &self.object.description {format!("\n    {}", description)} else {String::new()}
    )}
}