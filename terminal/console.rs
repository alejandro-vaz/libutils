//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::{
    problem::{
        Threaten,
        Threat
    },
    issue::Issue
};

//> HEAD -> STD
use std::collections::HashMap as Map;


//^
//^ CONSOLE
//^

//> CONSOLE -> TRAIT
pub trait Console: Threaten {
    fn arguments(&self) -> Vec<String>;
    fn environment(&self) -> Map<String, String>;
    fn sync(&mut self) -> ();
    fn problem<Object: Into<Issue>, const N: usize>(&mut self, threat: Threat<Object, N>) -> ();
}