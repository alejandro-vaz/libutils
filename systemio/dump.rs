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
    fn dump() -> SystemIO<Object>;
}