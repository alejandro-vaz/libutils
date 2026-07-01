//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    TERMINAL,
    Console
};


//^ 
//^ TESTS
//^ 

//> TESTS -> CLI
#[test]
fn cli() -> () {
    let _arguments = TERMINAL.read().arguments();
}