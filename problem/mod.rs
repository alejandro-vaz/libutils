//^
//^ HEAD
//^

//> HEAD -> MODULES
mod severity;

//> HEAD -> crate
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


//^
//^ PROBLEM
//^

//> PROBLEM -> STRUCT
pub struct Problem<Object: Into<Issue>> {
    pub chain: Vec<&'static str>,
    pub at: Instant,
    pub object: Object,
    pub severity: Severity
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