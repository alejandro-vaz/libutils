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
//^ SECTION
//^

//> SECTION -> ENUM
pub enum Section {
    Problem(Problem),
    Display(String),
    Debug(String)
}

//> SECTION -> DISPLAY
impl Display for Section {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Format {match self {
        Section::Problem(problem) => Display::fmt(problem, f),
        Section::Display(string) => Display::fmt(string, f),
        Section::Debug(string) => Display::fmt(string, f)
    }}
}