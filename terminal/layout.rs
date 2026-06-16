//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::report::Issue;


//^ 
//^ LAYOUT
//^ 

//> LAYOUT -> STRUCT
pub struct Layout {
    pub logs: Vec<Issue>
}

//> LAYOUT -> DEFAULT
impl const Default for Layout {
    fn default() -> Self {return Layout {
        logs: Vec::new()
    }}
}

//> LAYOUT -> IMPLEMENTATION
impl Layout {
    #[inline]
    pub fn view(&self) -> String {return self.logs.iter().map(|issue| issue.to_string()).collect::<Vec<String>>().join("\n\n")}
}