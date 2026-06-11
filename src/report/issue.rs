//^
//^ HEAD
//^

//> HEAD -> CORE
use core::fmt::{
    Display,
    Formatter,
    Result as Format
};


//^
//^ ISSUE
//^

//> ISSUE -> STRUCT
#[derive(Clone)]
pub struct Issue(pub &'static str);

//> ISSUE -> DISPLAY
impl Display for Issue {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {write!(formatter, "{}", self.0)}
}