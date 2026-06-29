//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(const_trait_impl)]
#![feature(const_convert)]

//> HEAD -> MODULES
mod severity;
#[cfg(test)]
mod tests;
mod threat;

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

//> HEAD -> ISSUE
use libutils_issue::Issue;


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