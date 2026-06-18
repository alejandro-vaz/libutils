//^ 
//^ LAYOUT
//^ 

//> LAYOUT -> STRUCT
pub struct Layout {
    pub logs: Vec<String>
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
    pub fn view(&self) -> String {return self.logs.join("\n\n")}
}