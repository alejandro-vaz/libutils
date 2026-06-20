//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    Report,
    toissue::ToIssue,
    shortcut::Attachment
};


//^
//^ NOTE
//^

//> NOTE -> TRAIT
pub trait Note {
    type Noted;
    fn note<'valid, Object: ToIssue>(self, report: &'valid mut Report<Object>, object: Object) -> Attachment<'valid, Self::Noted, Object>;
}

//> NOTE -> OPTION
impl<Type> Note for Option<Type> {
    type Noted = Type;
    fn note<'valid, Object: ToIssue>(self, report: &'valid mut Report<Object>, object: Object) -> Attachment<'valid, Self::Noted, Object> {
        if self.is_none() {report.warn(object)}
        return Attachment {
            report: Some(report),
            result: self
        }
    }
}