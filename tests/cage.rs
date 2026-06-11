//^
//^ HEAD
//^

//> HEAD -> API
use libutils::cage::Cage;

//> HEAD -> CORE
use core::ops::AddAssign;


//^
//^ TESTS
//^

//> TESTS -> INTEGER
#[test]
fn integer() -> () {
    static CAGE: Cage<usize> = Cage::new(0);
    CAGE.free(|x| x.add_assign(1));
    assert_eq!(CAGE.snapshot(), 1);
}

//> TESTS -> STRING
#[test]
fn string() -> () {
    let cage = Cage::new("hello".to_string());
    cage.free(|string| string.push_str(", world!"));
    assert_eq!(cage.release().as_str(), "hello, world!");
}