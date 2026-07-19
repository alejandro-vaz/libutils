//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::SystemIO;

//> HEAD -> ISSUING
use issuing::Issue;


//^
//^ DUMP
//^

//> DUMP -> TRAIT
pub trait Dump<Object: Into<Issue>> {
    type Argument;
    fn dump() -> SystemIO<Object, Self::Argument>;
}