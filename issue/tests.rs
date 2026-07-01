//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Issue;


//^
//^ TESTS
//^

//> TESTS -> CREATE
#[test]
fn create() -> () {
    let _issue = Issue::from("hello");
}