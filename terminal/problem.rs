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
    pub object: Object
}

//> PROBLEM -> DISPLAY
impl<Object: ToIssue> Display for Problem<Object> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {
        let issue = self.object.to_issue();
        return write!(
            formatter,
            "error ({}): {}{}", 
            self.chain.join(" < "),
            issue.name,
            if let Some(description) = issue.description {format!(
                "\n    {}",
                description
            )} else {String::new()}
        )
    }
}