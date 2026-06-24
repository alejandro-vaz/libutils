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
#[must_use]
pub struct Attachment<'valid, Type, Object: ToIssue, const NAME: &'static str> {
    pub report: Option<&'valid mut Report<Object, NAME>>,
    pub result: Option<Type>
}

//> TYPES -> BREAK
pub struct Break<'valid, Object: ToIssue, const NAME: &'static str>(Option<&'valid mut Report<Object, NAME>>);


//^
//^ FROMRESIDUAL
//^

//> FROMRESIDUAL -> ACT
impl<'valid, Type, Object: ToIssue, const NAME: &'static str> FromResidual<Break<'valid, Object, NAME>> for Act<Type, Object, NAME> {
    fn from_residual(residual: Break<'valid, Object, NAME>) -> Self {return Act {
        problems: replace(residual.0.unwrap(), Report {..}).problems,
        result: None
    }}
}

//> FROMRESIDUAL -> ATTACHMENT
impl<'valid, Type, Object: ToIssue, const NAME: &'static str> FromResidual<Break<'valid, Object, NAME>> for Attachment<'valid, Type, Object, NAME> {
    fn from_residual(residual: Break<'valid, Object, NAME>) -> Self {return Attachment {
        report: residual.0,
        result: None
    }}
}


//^
//^ RESIDUAL
//^

//> RESIDUAL -> BREAK
impl<'valid, Type, Object: ToIssue, const NAME: &'static str> Residual<Type> for Break<'valid, Object, NAME> {
    type TryType = Attachment<'valid, Type, Object, NAME>;
}


//^
//^ TRY
//^

//> TRY -> ATTACHMENT
impl<'valid, Type, Object: ToIssue, const NAME: &'static str> Try for Attachment<'valid, Type, Object, NAME> {
    type Output = Type;
    type Residual = Break<'valid, Object, NAME>;
    fn from_output(output: Self::Output) -> Self {return Attachment {
        report: None,
        result: Some(output)
    }}
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {return match self.result {
        Some(value) => ControlFlow::Continue(value),
        None => ControlFlow::Break(Break(self.report))
    }}
}