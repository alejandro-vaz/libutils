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
    mem::transmute_neo as transmute,
    marker::Destruct
};


//^
//^ TYPES
//^

//> TYPES -> BREAK
pub struct Break;


//^
//^ FROMRESIDUAL
//^

//> FROMRESIDUAL -> ACT
const impl<Type> FromResidual<Break> for Act<Type> {
    fn from_residual(_residual: Break) -> Self {return unsafe {transmute(None::<Type>)}}
}


//^
//^ RESIDUAL
//^

//> RESIDUAL -> BREAK
const impl<Type: [const] Destruct> Residual<Type> for Break {
    type TryType = Act<Type>;
}


//^
//^ TRY
//^

//> TRY -> ATTACHMENT
const impl<Type: [const] Destruct> Try for Act<Type> {
    type Output = Type;
    type Residual = Break;
    fn from_output(output: Self::Output) -> Self {return unsafe {transmute(Some(output))}}
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {return if let Some(value) = self.value() {
        ControlFlow::Continue(value)
    } else {ControlFlow::Break(Break)}}
}