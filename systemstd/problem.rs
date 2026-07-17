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


//^
//^ PROBLEM
//^

//> PROBLEM -> STRUCT
pub struct Problem<'valid> {
    pub chain: &'valid [&'static str],
    pub issue: Issue,
    pub severity: Option<bool>
}

//> PROBLEM -> DISPLAY
impl<'valid> Display for Problem<'valid> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {write!(
        formatter,
        "{}[bold]: {}[/]{}",
        match self.severity {
            None => "[bold red]Critical[/]",
            Some(false) => "[bold red]Warning[/]",
            Some(true) => "[bold yellow]Error[/]"
        },
        self.issue.name,
        format!(
            "{}{}{}{}",
            (!self.chain.is_empty()).then(|| {
                format!("\n[bold purple]@[/] [purple]{}[/]", self.chain.join(" > "))
            }).unwrap_or_default(),
            self.issue.traceback.as_ref().map(|string| {
                format!("\n[gray]traceback[/]: {string}")
            }).unwrap_or_default(),
            self.issue.description.as_ref().map(|string| {
                format!("\n{string}")
            }).unwrap_or_default(),
            self.issue.help.as_ref().map(|string| {
                format!("\n[blue]help[/]: {string}")
            }).unwrap_or_default()
        ).replace('\n', "\n    ")
    )}
}