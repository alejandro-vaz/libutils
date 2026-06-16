//^
//^ HEAD
//^

//> HEAD -> STD
use std::io::{
    Write, 
    stderr
};

//> HEAD -> SUPER
use super::diff::Diff;


//^
//^ HANDLER
//^

//> HANDLER -> STRUCT
pub struct Handler;

//> HANDLER -> IMPLEMENTATION
impl Handler {
    #[inline]
    pub fn sync(&self, diff: Diff) -> () {stderr().lock().write(<Diff as Into<Vec<u8>>>::into(diff).as_ref()).unwrap();}
}