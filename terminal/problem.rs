//^
//^ HEAD
//^

//> HEAD -> THREAT
use libutils_threat::{
    Severity,
    Threat
};

//> HEAD -> STD
use std::time::Instant;

//> HEAD -> ISSUE
use libutils_issue::Issue;

//> HEAD -> CORE
use core::fmt::{
    Result as Format,
    Formatter,
    Display
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
        "@ {}\n{}: {}{}",
        self.chain.join(" > "),
        <&Severity as Into<&'static str>>::into(&self.severity),
        self.issue.name,
        if let Some(description) = &self.issue.description {format!("\n    {}", description)} else {String::new()}
    )}
}

//> PROBLEM -> FROM THREAT
impl<Object: Into<Issue>> From<Threat<Object>> for Problem {
    fn from(value: Threat<Object>) -> Self {return Self {
        chain: value.chain,
        at: Instant::now(),
        issue: value.object.into(),
        severity: value.severity
    }}
}