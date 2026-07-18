//^
//^ HEAD
//^

//> HEAD -> SYSTEMSTD
use systemstd::{
    Argument, 
    CliType, 
    IoError, 
    System,
    OpenMode
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
    System::print("value");
}

//> TESTS -> ARGUMENTS
#[test]
fn arguments() -> () {
    assert_matches!("myexec.exect".to_string().try_into(), Ok(Argument::Path {..}));
    assert_matches!("rm".to_string().try_into(), Ok(Argument::Target {..}));
    assert_matches!("-rf".to_string().try_into(), Ok(Argument::Alias {..}));
    assert_matches!("--please".to_string().try_into(), Ok(Argument::Flag {..}));
    assert_matches!("--opt=3".to_string().try_into(), Ok(Argument::Setting {
        value: CliType::Integer {
            value: 3
        },
        ..
    }));
    assert_matches!(TryInto::<Argument>::try_into("--key=impossible".to_string()), Err(
        IoError::UnknownSettingValue {..}
    ));
    assert_matches!(TryInto::<Argument>::try_into("&".to_string()), Err(
        IoError::FailureParsingArgument {..}
    ));
}

//> TESTS -> READ
#[test]
fn read() -> () {assert_eq!(
    System::path("Cargo.toml").file::<{OpenMode::Read}>(None).unwrap().read_bytes().unwrap().into_iter().next(),
    Some(b'[')
)}