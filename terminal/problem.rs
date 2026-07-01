//^
//^ HEAD
//^

//> HEAD -> THREAT
use libutils_threat::Threat;

//> HEAD -> ISSUE
use libutils_issue::{
    Issue,
    Severity
};

//> HEAD -> CORE
use core::fmt::{
    Result as Format,
    Formatter,
    Display
};

//> HEAD -> STD
use std::time::Instant;


//^
//^ PROBLEM
//^

//> PROBLEM -> STRUCT
pub struct Problem {
    pub chain: Vec<&'static str>,
    pub issue: Issue,
    pub _at: Instant
}

//> PROBLEM -> DISPLAY
impl Display for Problem {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {write!(
        formatter,
        "@ {}\n{}: {}{}\n",
        self.chain.join(" > "),
        <&Severity as Into<&'static str>>::into(&self.issue.severity),
        self.issue.name,
        if let Some(description) = &self.issue.description {format!("\n    {}", description)} else {String::new()}
    )}
}

//> PROBLEM -> FROM THREAT
impl From<Threat> for Problem {
    fn from(value: Threat) -> Self {return Self {
        chain: value.chain,
        issue: value.issue,
        _at: Instant::now()
    }}
}