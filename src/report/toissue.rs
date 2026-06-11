//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::issue::Issue;


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
    fn to_issue(&self) -> Issue {return Issue(*self)}
}