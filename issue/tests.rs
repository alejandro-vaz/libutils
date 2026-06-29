//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Issue;

//> HEAD -> ALLOC
use alloc::string::ToString;


//^
//^ TESTS
//^

//> TESTS -> CREATE
#[test]
fn create() -> () {
    let _issue = Issue {
        name: "die",
        description: Some("please".to_string())
    };
}