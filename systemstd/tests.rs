//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::System;

//> HEAD -> SYSTEMIO
use systemio::{
    SystemIO, 
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
    let _arguments = System::arguments();
    System::print("value").ignore();
    let mut file = System::open("myfile.txt").unwrap();
    let _content = file.read().unwrap();
}