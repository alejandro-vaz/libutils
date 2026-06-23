//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::problem::Problem;

//> HEAD -> CRATE
use crate::report::Issue;


//^ 
//^ LAYOUT
//^ 

//> LAYOUT -> STRUCT
pub struct Layout {
    pub problems: Vec<Problem<Issue>> = Vec::new()
}

//> LAYOUT -> IMPLEMENTATION
impl Layout {
    #[inline]
    pub fn view(&self) -> String {return self.problems.iter().map(|problem| problem.to_string()).collect::<Vec<String>>().join("\n\n")}
}