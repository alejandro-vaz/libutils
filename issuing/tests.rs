//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(default_field_values)]

//> HEAD -> ISSUING
use issuing::Issue;


//^
//^ TESTS
//^

//> TESTS -> CREATE
#[test]
fn create() -> () {
    let _issue = Issue::from("hello");
    let _manual = Issue {
        name: "myname",
        description: None,
        ..
    };
}