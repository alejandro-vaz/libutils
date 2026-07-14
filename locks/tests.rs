//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Mutex;

//> HEAD -> CORE
use core::ops::AddAssign;


//^
//^ TESTS
//^

//> TESTS -> INTEGER
#[test]
fn integer() -> () {
    static CAGE: Mutex<usize> = Mutex::default();
    CAGE.get(|x| x.add_assign(1));
    assert_eq!(CAGE.cloned(), 1);
    CAGE.replace(0);
    assert_eq!(CAGE.cloned(), 0);
}

//> TESTS -> STRING
#[test]
fn string() -> () {
    let cage = Mutex::new("hello");
    cage.replace("hello, world!");
    let _ = cage.get(|string| string.len());
    cage.get(|string| assert_eq!(*string, "hello, world!"));
    let _final = cage.release();
}