//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::{
    issue::Issue,
    problem::Problem
};

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
    fn issue<Object: Into<Issue>>(&mut self, problem: Problem<Object>) -> ();
}