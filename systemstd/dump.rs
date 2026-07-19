//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    System,
    argument::Argument
};

//> HEAD -> SYSTEMIO
use systemio::{
    Dump,
    SystemIO
};

//> HEAD -> ISSUING
use issuing::Issue;


//^
//^ DUMP
//^

//> DUMP -> IMPLEMENTATION
impl<Object: Into<Issue>> Dump<Object> for System {
    type Argument = Argument;
    fn dump() -> SystemIO<Object, Self::Argument> {return SystemIO {
        arguments: || Self::arguments(),
        warning: |object, chain| Self::warning(object, chain),
        error: |object, chain| Self::error(object, chain),
        critical: |object, chain| Self::critical(object, chain)
    }}
}