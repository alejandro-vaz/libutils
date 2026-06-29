//^ 
//^ HEAD
//^ 

//> HEAD -> SUPER
use super::{
    Problem,
    Severity
};

//> HEAD -> STD
use std::time::Instant;


//^
//^ TESTS
//^

//> TESTS -> CREATION
#[test]
fn creation() -> () {
    let _ = Problem {
        chain: Vec::from(["a", "bcd"]),
        at: Instant::now(),
        issue: "bye".into(),
        severity: Severity::Warning
    };
}