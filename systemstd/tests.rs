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

//> TESTS -> ERASE
#[test]
fn erase() -> () {
    System::print("hello").sync();
    System::clear().sync();
}

//> TESTS -> READ
#[test]
fn read() -> () {
    assert_eq!(System::open("Cargo.toml").unwrap().read_bytes().unwrap().into_iter().next(), Some(b'['));
}