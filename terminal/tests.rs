//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::TERMINAL;

//> HEAD -> CONSOLE
use libutils_console::Console;


//^ 
//^ TESTS
//^ 

//> TESTS -> CLI
#[test]
fn cli() -> () {
    let _arguments = TERMINAL.read().arguments();
}