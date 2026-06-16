//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::issue::Issue;

//> HEAD -> CORE
use std::process::ExitCode;


//^ 
//^ TOISSUE
//^ 

//> TOISSUE -> TRAIT
pub trait ToIssue {
    fn to_issue(&self) -> Issue;
}

//> TOISSUE -> ISSUE
impl ToIssue for Issue {
    #[inline]
    fn to_issue(&self) -> Issue {return self.clone()}
}

//> TOISSUE -> &'STATIC STR
impl ToIssue for &'static str {
    fn to_issue(&self) -> Issue {return Issue {
        name: *self,
        code: ExitCode::from(*self as *const str as *const u8 as usize as u8),
        description: None
    }}
}