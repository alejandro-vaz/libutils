//^
//^ HEAD
//^

//> HEAD -> ISSUING
use issuing::Issue;

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
        "@ {}\n{:#?}: {}{}\n",
        self.chain.join(" > "),
        self.issue.severity,
        self.issue.name,
        self.issue.description.as_ref().map(|description| format!("\n    {description}")).unwrap_or_default()
    )}
}