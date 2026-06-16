//^
//^ HEAD
//^

//> HEAD -> CORE
use core::fmt::{
    Display,
    Formatter,
    Result as Format
};

//> HEAD -> STD
use std::process::ExitCode;


//^
//^ ISSUE
//^

//> ISSUE -> STRUCT
#[derive(Clone)]
pub struct Issue {
    pub name: &'static str,
    pub code: ExitCode,
    pub description: Option<String>
}

//> ISSUE -> DISPLAY
impl Display for Issue {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {return write!(
        formatter,
        "error: {}{}", 
        self.name,
        if let Some(description) = &self.description {format!(
            "\n    > {}",
            description
        )} else {String::new()}
    )}
}