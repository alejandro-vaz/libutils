//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Cage;

//> HEAD -> CORE
use core::ops::AddAssign;


//^
//^ TESTS
//^

//> TESTS -> INTEGER
#[test]
fn integer() -> () {
    static CAGE: Cage<usize> = Cage::new(0);
    CAGE.write(|x| x.add_assign(1));
    assert_eq!(CAGE.cloned(), 1);
    let _copy = CAGE.get();
}

//> TESTS -> STRING
#[test]
fn string() -> () {
    let cage = Cage::new("hello".to_string());
    cage.write(|string| string.push_str(", world!"));
    let _ = cage.read(|string| string.len());
    cage.read(|string| assert_eq!(string.as_str(), "hello, world!"));
    let _clone = cage.cloned();
}