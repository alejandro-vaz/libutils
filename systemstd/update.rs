//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    LAYOUT,
    OUTPUT
};

//> HEAD -> STD
use std::io::{
    stdout,
    Write
};

//> HEAD -> BYTEDIFF
use bytediff::Diff;


//^
//^ UPDATE
//^

//> UPDATE -> STRUCT
pub struct Update;

//> UPDATE -> TRAIT
impl Update {
    pub fn sync(self) -> () {
        let mut content = LAYOUT.with(|layout| {
            layout.iter().map(ToString::to_string).collect::<Vec<String>>()
        }).join("\n\n");
        content.push('\n');
        OUTPUT.with(|output| {
            stdout().write_all(<Diff as Into<Vec<u8>>>::into(Diff::new(
                output.as_bytes(), 
                content.as_bytes()
            )).as_ref()).unwrap();
            stdout().flush().unwrap();
            *output = content;
        });
    }
    pub fn ignore(self) -> () {}
}
