//^ 
//^ HEAD
//^ 

//> HEAD -> SUPER
use super::{
    Report,
    toissue::ToIssue,
    Act
};

//> HEAD -> CORE
use core::{
    mem::replace,
    ops::{
        FromResidual,
        Residual,
        Try,
        ControlFlow
    }
};


//^
//^ TYPES
//^

//> TYPES -> ATTACHMENT
pub struct Attachment<'valid, Type, Object: ToIssue> {
    pub report: Option<&'valid mut Report<Object>>,
    pub result: Option<Type>
}

//> TYPES -> BREAK
pub struct Break<'valid, Object: ToIssue>(Option<&'valid mut Report<Object>>);


//^
//^ FROMRESIDUAL
//^

//> FROMRESIDUAL -> ACT
impl<'valid, Type, Object: ToIssue> FromResidual<Break<'valid, Object>> for Act<Type, Object> {
    fn from_residual(residual: Break<'valid, Object>) -> Self {return unsafe {replace(residual.0.unwrap(), Report {..}).abort()}}
}

//> FROMRESIDUAL -> ATTACHMENT
impl<'valid, Type, Object: ToIssue> FromResidual<Break<'valid, Object>> for Attachment<'valid, Type, Object> {
    fn from_residual(residual: Break<'valid, Object>) -> Self {return Attachment {
        report: residual.0,
        result: None
    }}
}


//^
//^ RESIDUAL
//^

//> RESIDUAL -> BREAK
impl<'valid, Type, Object: ToIssue> Residual<Type> for Break<'valid, Object> {
    type TryType = Attachment<'valid, Type, Object>;
}


//^
//^ TRY
//^

//> TRY -> ATTACHMENT
impl<'valid, Type, Object: ToIssue> Try for Attachment<'valid, Type, Object> {
    type Output = Type;
    type Residual = Break<'valid, Object>;
    fn from_output(output: Self::Output) -> Self {return Attachment {
        report: None,
        result: Some(output)
    }}
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {return match self.result {
        Some(value) => ControlFlow::Continue(value),
        None => ControlFlow::Break(Break(self.report))
    }}
}