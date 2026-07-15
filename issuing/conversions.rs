//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Issue;


//^
//^ IMPLEMENTATIONS
//^

//> IMPLEMENTATIONS -> &'STATIC STR
const impl From<&'static str> for Issue {
    fn from(value: &'static str) -> Self {return Self {
        name: value,
        ..
    }}
}