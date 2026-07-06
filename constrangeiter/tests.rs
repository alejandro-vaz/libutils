//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::ConstIntoIterator;

//> HEAD -> ALLOC
use alloc::vec::Vec;

//> HEAD -> CORE
use core::array::from_fn as arrayfn;


//^
//^ TESTS
//^

//> TESTS -> RANGE
#[test]
fn range() -> () {
    assert_eq!(&[0, 1, 2, 3, 4], Vec::from_iter((0..5).const_into_iter()).as_slice());
    assert_eq!(&[-1, 0, 1], Vec::from_iter((-1..2).const_into_iter()).as_slice());
    assert_eq!(&[] as &[i32], Vec::from_iter((6..2).const_into_iter()).as_slice());
}

//> TESTS -> RANGEINCLUSIVE
#[test]
fn rangeinclusive() -> () {
    assert_eq!(&[0, 1, 2, 3, 4], Vec::from_iter((0..=4).const_into_iter()).as_slice());
    assert_eq!(&[-1, 0, 1], Vec::from_iter((-1..=1).const_into_iter()).as_slice());
    assert_eq!(&[] as &[i32], Vec::from_iter((6..=2).const_into_iter()).as_slice());
}

//> TESTS -> RANGEFROM
#[test]
fn rangefrom() -> () {
    let array = arrayfn::<u8, 256, _>(|index| index as u8);
    assert_eq!(&array, Vec::from_iter((0u8..).const_into_iter()).as_slice())
}