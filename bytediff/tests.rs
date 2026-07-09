//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Diff;

//> HEAD -> ALLOC
use alloc::vec::Vec;


//^
//^ DIFF
//^

//> DIFF -> CREATION
#[test]
fn creation() -> () {
    let diff = Diff::new("hello".as_bytes(), "hellos".as_bytes());
    assert_eq!(<Diff<'_> as Into<Vec<u8>>>::into(diff), [b's'])
}

//> DIFF -> REMOVAL
#[test]
fn removal() -> () {
    let diff = Diff::new("hellos".as_bytes(), "hello".as_bytes());
    assert_eq!(<Diff<'_> as Into<Vec<u8>>>::into(diff), [b'\x1B', b'[', b'D', b' ', b'\x1B', b'[', b'D'])
}