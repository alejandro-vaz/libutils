//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::TERMINAL;

//> HEAD -> CONSOLE
use libutils_console::{
    Console, 
    Synchronization,
    Descriptor
};


//^ 
//^ TESTS
//^ 

//> TESTS -> CLI
#[test]
#[should_panic]
fn cli() -> () {
    let _arguments = TERMINAL.arguments();
    TERMINAL.print("value").ignore();
    let mut file = TERMINAL.open("myfile.txt").unwrap();
    let _content = file.read().unwrap();
}