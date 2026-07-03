//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::problem::Problem;

//> HEAD -> CORE
use core::fmt::{
    Display,
    Formatter,
    Result as Format
};


//^
//^ ITEM
//^

//> ITEM -> ENUM
pub enum Item {
    Problem(Problem),
    String(String)
}

//> ITEM -> DISPLAY
impl Display for Item {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Format {match self {
        Item::Problem(problem) => Display::fmt(problem, f),
        Item::String(string) => Display::fmt(string, f),
    }}
}