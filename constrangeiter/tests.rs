//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(new_range)]

//> HEAD -> CONSTRANGEITER
use constrangeiter::ConstIntoIterator;

//> HEAD -> CORE
use core::array::from_fn as arrayfn;


//^
//^ TESTS
//^

//> TESTS -> RANGE
#[test]
fn range() -> () {
    let iterator = (0..5).const_into_iter();
    assert_eq!(iterator.len(), 5);
    assert_eq!(&[0, 1, 2, 3, 4], Vec::from_iter(iterator).as_slice());
    assert_eq!(&[-1, 0, 1], Vec::from_iter((-1..2).const_into_iter()).as_slice());
    assert_eq!(&[] as &[i32], Vec::from_iter((6..2).const_into_iter()).as_slice());
}

//> TESTS -> RANGEINCLUSIVE
#[test]
fn rangeinclusive() -> () {
    let iterator = (0..=4).const_into_iter();
    assert_eq!(iterator.len(), 5);
    assert_eq!(&[0, 1, 2, 3, 4], Vec::from_iter(iterator).as_slice());
    assert_eq!(&[-1, 0, 1], Vec::from_iter((-1..=1).const_into_iter()).as_slice());
    assert_eq!(&[] as &[i32], Vec::from_iter((6..=2).const_into_iter()).as_slice());
}

//> TESTS -> RANGEFROM
#[test]
fn rangefrom() -> () {
    let array = arrayfn::<u8, 256, _>(|index| index as u8);
    let iterator = (0u8..).const_into_iter();
    assert_eq!(iterator.len(), 256);
    assert_eq!(&array, Vec::from_iter(iterator).as_slice())
}