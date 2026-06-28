//^
//^ HEAD
//^

//> HEAD -> MODULES
mod severity;
mod threat;

//> HEAD -> CRATE
use crate::issue::Issue;

//> HEAD -> STD
use std::time::Instant;

//> HEAD -> CORE
use core::fmt::{
    Display,
    Formatter,
    Result as Format
};

//> HEAD -> SEVERITY
pub use severity::Severity;

//> HEAD -> THREAT
pub use threat::{
    Threat,
    Threaten
};


//^
//^ PROBLEM
//^

//> PROBLEM -> STRUCT
pub struct Problem {
    pub chain: Vec<&'static str>,
    pub at: Instant,
    pub issue: Issue,
    pub severity: Severity
}

//> PROBLEM -> DISPLAY
impl Display for Problem {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {write!(
        formatter,
        "@ {}\n{:?}: {}{}",
        self.chain.iter().rev().map(|node| *node).collect::<Vec<&'static str>>().join(" > "),
        <&Severity as Into<&'static str>>::into(&self.severity),
        self.issue.name,
        if let Some(description) = &self.issue.description {format!("\n    {}", description)} else {String::new()}
    )}
}