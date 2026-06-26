//^
//^ HEAD
//^

//> HEAD -> STD
use std::process::{
    ExitCode,
    Termination
};

//> HEAD -> CORE
use core::mem::transmute_neo as transmute;


//^
//^ ACT
//^

//> ACT -> STRUCT
pub struct Act<Type> {
    option: Option<Type>
}

//> ACT -> TERMINATION
impl Termination for Act<()> {
    fn report(self) -> ExitCode {return ExitCode::SUCCESS}
}

//> ACT -> IMPLEMENTATION
impl<Type> Act<Type> {
    #[inline]
    pub fn map<Return, Closure: FnOnce(Type) -> Return>(self, closure: Closure) -> Act<Return> {return Act {
        option: self.option.map(closure)
    }}
}

//> ACT -> INTO
impl<Type> const Into<Option<Type>> for Act<Type> {
    fn into(self) -> Option<Type> {return unsafe {transmute(self)}}
}