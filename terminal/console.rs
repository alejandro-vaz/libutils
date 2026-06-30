//^
//^ HEAD
//^

//> HEAD -> HASHBROWN
use hashbrown::HashMap as Map;

//> HEAD -> PROBLEM
use libutils_threat::Threat;

//> HEAD -> ISSUE
use libutils_issue::Issue;


//^
//^ CONSOLE
//^

//> CONSOLE -> TRAIT
pub trait Console {
    fn arguments(&self) -> Vec<String>;
    fn environment(&self) -> Map<String, String>;
    fn sync(&mut self) -> ();
    fn problem<Object: Into<Issue>>(&mut self, threat: Threat<Object>) -> ();
}