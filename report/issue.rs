//^
//^ HEAD
//^

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