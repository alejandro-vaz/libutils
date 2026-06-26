//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::{
    issue::Issue,
    problem::Problem,
    log::Log
};


//^ 
//^ LAYOUT
//^ 

//> LAYOUT -> STRUCT
pub struct Layout {
    pub problems: Log<Problem<Issue>> = Log::new()
}

//> LAYOUT -> IMPLEMENTATION
impl Layout {
    #[inline]
    pub fn view(&self) -> String {return self.problems.iter().map(|problem| problem.to_string()).collect::<Vec<String>>().join("\n\n")}
}