//^
//^ HEAD
//^

//> HEAD -> HASHBROWN
use hashbrown::HashMap as Map;

//> HEAD -> PROBLEM
use libutils_problem::{
    Threat,
    Threaten
};

//> HEAD -> ISSUE
use libutils_issue::Issue;


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