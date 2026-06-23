//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::problem::Problem;

//> HEAD -> CRATE
use crate::report::ToIssue;

//> HEAD -> STD
use std::collections::HashMap as Map;


//^
//^ CONSOLE
//^

//> CONSOLE -> TRAIT
pub trait Console {
    fn arguments(&self) -> Vec<String>;
    fn environment(&self) -> Map<String, String>;
    fn render(&mut self) -> ();
    fn error<Object: ToIssue>(&mut self, problem: &Problem<Object>) -> ();
}