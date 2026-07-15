//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(const_trait_impl)]
#![feature(const_default)]

//> HEAD -> LOCKS
use locks::Mutex;

//> HEAD -> CORE
use core::ops::AddAssign;


//^
//^ TESTS
//^

//> TESTS -> INTEGER
#[test]
fn integer() -> () {
    static CAGE: Mutex<usize> = Mutex::default();
    CAGE.with(|x| x.add_assign(1));
    assert_eq!(CAGE.cloned(), 1);
    CAGE.replace(0);
    assert_eq!(CAGE.cloned(), 0);
}

//> TESTS -> STRING
#[test]
fn string() -> () {
    let cage = Mutex::new("hello");
    cage.replace("hello, world!");
    let _ = cage.with(|string| string.len());
    cage.with(|string| assert_eq!(*string, "hello, world!"));
    let _final = cage.release();
}

//> TESTS -> GET
#[test]
fn get() -> () {
    let cage = Mutex::new("hello");
    let guard = cage.get();
    assert!(cage.try_get().is_none());
    assert_eq!(guard.to_string(), String::from("hello"));
}
