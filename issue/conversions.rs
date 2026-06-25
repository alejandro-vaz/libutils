//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Issue;


//^
//^ IMPLEMENTATIONS
//^

//> IMPLEMENTATIONS -> &'STATIC STR
impl Into<Issue> for &'static str {
    fn into(self) -> Issue {return Issue {
        name: self,
        description: None
    }}
}