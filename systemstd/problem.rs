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
    pub severity: Option<bool>,
    pub _at: Instant
}

//> PROBLEM -> DISPLAY
impl Display for Problem {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {write!(
        formatter,
        "[purple bold]@[/] [purple]{}[/]\n{}: {}{}",
        self.chain.join(" > "),
        match self.severity {
            None => "[bold red]Critical[/]",
            Some(false) => "[red]Warning[/]",
            Some(true) => "[yellow]Error[/]"
        },
        self.issue.name,
        self.issue.description.as_ref().map(|description| format!("\n    {description}")).unwrap_or_default()
    )}
}