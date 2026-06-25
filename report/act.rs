//^
//^ HEAD
//^

//> HEAD -> STD
use std::process::{
    ExitCode,
    Termination
};


//^
//^ ACT
//^

//> ACT -> STRUCT
pub struct Act<Type, const NAME: &'static str>(pub Option<Type>);

//> ACT -> TERMINATION
impl Termination for Act<(), "Main"> {
    fn report(self) -> ExitCode {return ExitCode::SUCCESS}
}

//> ACT -> IMPLEMENTATION
impl<Type, const NAME: &'static str> Act<Type, NAME> {
    #[inline]
    pub fn map<Return, Closure: FnOnce(Type) -> Return>(self, closure: Closure) -> Act<Return, NAME> {return Act(self.0.map(closure))}
}