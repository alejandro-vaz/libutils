//^ 
//^ HEAD
//^ 

//> HEAD -> SUPER
use super::act::Act;

//> HEAD -> CORE
use core::{
    ops::{
        FromResidual,
        Residual,
        Try,
        ControlFlow
    },
    convert::Infallible
};


//^
//^ TYPES
//^

//> TYPES -> BREAK
pub struct Break<const NAME: &'static str>;


//^
//^ FROMRESIDUAL
//^

//> FROMRESIDUAL -> ACT
impl<Type, const NAME: &'static str, const OTHER: &'static str> FromResidual<Break<NAME>> for Act<Type, OTHER> {
    fn from_residual(_residual: Break<NAME>) -> Self {return Act(None)}
}

//> FROMRESIDUAL -> OPTION
impl<Type, const NAME: &'static str> FromResidual<Option<Infallible>> for Act<Type, NAME> {
    fn from_residual(_residual: Option<Infallible>) -> Self {return Act(None)}
}

//> FROMRESIDUAL -> RESULT
impl<Type, Error, const NAME: &'static str> FromResidual<Result<Infallible, Error>> for Act<Type, NAME> {
    fn from_residual(_residual: Result<Infallible, Error>) -> Self {return Act(None)}
}


//^
//^ RESIDUAL
//^

//> RESIDUAL -> BREAK
impl<Type, const NAME: &'static str> Residual<Type> for Break<NAME> {
    type TryType = Act<Type, NAME>;
}


//^
//^ TRY
//^

//> TRY -> ATTACHMENT
impl<Type, const NAME: &'static str> Try for Act<Type, NAME> {
    type Output = Type;
    type Residual = Break<NAME>;
    fn from_output(output: Self::Output) -> Self {return Act(Some(output))}
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {return match self.0 {
        None => ControlFlow::Break(Break),
        Some(value) => ControlFlow::Continue(value)
    }}
}