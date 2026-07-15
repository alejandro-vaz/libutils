//^
//^ HEAD
//^

//> HEAD -> SYSTEMSTD
use systemstd::{
    System,
    Argument
};

//> HEAD -> STD
use std::assert_matches;


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

//> TESTS -> ARGUMENTS
#[test]
fn arguments() -> () {
    let args = ["myexec.exect", "rm", "-rf", "--please", "--opt=true", "&"].map(ToString::to_string).map(Into::into);
    assert_matches!(args, [
        Argument::Path(_),
        Argument::Target(_),
        Argument::Alias(_),
        Argument::Flag(_),
        Argument::Setting(_, _),
        Argument::Unknown(_)
    ]);
}

//> TESTS -> READ
#[test]
fn read() -> () {
    assert_eq!(System::open("Cargo.toml").unwrap().read_bytes().unwrap().into_iter().next(), Some(b'['));
}