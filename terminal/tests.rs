//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Terminal;

//> HEAD -> CONSOLE
use libutils_console::{
    Console, 
    Update,
    Descriptor
};


//^ 
//^ TESTS
//^ 

//> TESTS -> CLI
#[test]
#[should_panic]
fn cli() -> () {
    let _arguments = Terminal::arguments();
    Terminal::print("value").ignore();
    let mut file = Terminal::open("myfile.txt").unwrap();
    let _content = file.read().unwrap();
}