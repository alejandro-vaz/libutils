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
const impl From<&'static str> for Issue {
    #[inline]
    fn from(value: &'static str) -> Self {return Self {
        name: value,
        description: None,
        severity: Severity::Critical
    }}
}