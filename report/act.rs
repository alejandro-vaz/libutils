//^
//^ HEAD
//^

//> HEAD -> STD
use std::{marker::Destruct, process::{
    ExitCode,
    Termination
}};


//^
//^ ACT
//^

//> ACT -> STRUCT
pub struct Act<Type> {
    option: Option<Type>
}

//> ACT -> TERMINATION
impl Termination for Act<()> {
    fn report(self) -> ExitCode {return match self.option {
        None => ExitCode::FAILURE,
        Some(()) => ExitCode::SUCCESS
    }}
}

//> ACT -> IMPLEMENTATION
const impl<Type: [const] Destruct> Act<Type> {
    #[inline]
    pub fn map<Return, Closure: [const] FnOnce(Type) -> Return + [const] Destruct>(self, closure: Closure) -> Act<Return> {return Act {
        option: self.option.map(closure)
    }}
    #[inline]
    pub fn value(self) -> Option<Type> {return self.option}
}