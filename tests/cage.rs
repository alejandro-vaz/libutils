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
    CAGE.write().add_assign(1);
    assert_eq!(CAGE.read().clone(), 1);
}

//> TESTS -> STRING
#[test]
fn string() -> () {
    let cage = Cage::new("hello".to_string());
    cage.write().push_str(", world!");
    let _ = cage.read().len();
    assert_eq!(cage.release().as_str(), "hello, world!");
}