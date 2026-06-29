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
    let _vars = TERMINAL.read().environment();
}