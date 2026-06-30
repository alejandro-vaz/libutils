//^
//^ HEAD
//^

//> HEAD -> PROBLEM
use libutils_threat::Threat;

//> HEAD -> SUPER
use super::argument::Argument;


//^
//^ CONSOLE
//^

//> CONSOLE -> TRAIT
pub trait Console {
    fn arguments(&self) -> &[Argument];
    fn sync(&mut self) -> ();
    fn problem(&mut self, threat: Threat) -> ();
}