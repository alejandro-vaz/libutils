//^
//^ HEAD
//^

//> HEAD -> ISSUING
use issuing::Issue;

//> HEAD -> SUPER
use super::argument::Argument;


//^
//^ DUMP
//^

//> DUMP -> STRUCT
pub struct Dump<Object: Into<Issue>> {
    pub arguments: fn() -> &'static [Argument],
    pub warning: fn(Object, &[&'static str]) -> (),
    pub error: fn(Object, &[&'static str]) -> (),
    pub critical: fn(Object, &[&'static str]) -> !
}