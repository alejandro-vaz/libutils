//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::TERMINAL;

//> HEAD -> CONSOLE
use libutils_console::{Console, Handle};


//^ 
//^ TESTS
//^ 

//> TESTS -> CLI
#[test]
fn cli() -> () {
    let _arguments = TERMINAL.arguments();
    TERMINAL.print("value").ignore();
}