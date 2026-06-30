//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    Issue,
    severity::Severity
};


//^
//^ IMPLEMENTATIONS
//^

//> IMPLEMENTATIONS -> &'STATIC STR
const impl Into<Issue> for &'static str {
    fn into(self) -> Issue {return Issue {
        name: self,
        description: None,
        severity: Severity::Critical
    }}
}