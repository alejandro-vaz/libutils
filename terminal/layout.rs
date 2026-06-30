//^
//^ HEAD
//^

//> HEAD -> LOG
use libutils_log::Log;

//> HEAD -> SUPER
use super::problem::Problem;


//^ 
//^ LAYOUT
//^ 

//> LAYOUT -> STRUCT
pub struct Layout {
    pub problems: Log<Problem> = Log::new()
}

//> LAYOUT -> IMPLEMENTATION
impl Layout {
    #[inline]
    pub fn view(&self) -> String {return self.problems.iter().map(|problem| problem.to_string()).collect::<Vec<String>>().join("\n\n")}
}