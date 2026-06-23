//^
//^ HEAD
//^

//> HEAD -> crate
use crate::report::ToIssue;

//> HEAD -> STD
use std::time::Instant;

//> HEAD -> CORE
use core::fmt::{
    Display,
    Formatter,
    Result as Format
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
impl<Object: ToIssue> Display for Problem<Object> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {
        let issue = self.object.to_issue();
        return write!(
            formatter,
            "{:?} ({}): {}{}", 
            self.severity,
            self.chain.iter().rev().map(|node| *node).collect::<Vec<&'static str>>().join(" > "),
            issue.name,
            if let Some(description) = issue.description {format!(
                "\n    {}",
                description
            )} else {String::new()}
        )
    }
}