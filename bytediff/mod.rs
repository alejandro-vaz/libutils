//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod conversions;


//^ 
//^ DIFF
//^ 

//> DIFF -> STRUCT
#[derive(Debug, Clone, Copy)]
pub struct Diff<'valid> {
    remove: usize,
    write: &'valid [u8]
}

//> DIFF -> IMPLEMENTATION
impl<'valid> Diff<'valid> {
    pub fn new(current: &[u8], replacement: &'valid [u8]) -> Self {
        let mut old = current.into_iter();
        let mut new = replacement.into_iter();
        for index in 0.. {return if old.next() != new.next() {Diff {
            remove: current.len() - index,
            write: &replacement[index..]
        }} else {continue}}
        unreachable!();
    }
}