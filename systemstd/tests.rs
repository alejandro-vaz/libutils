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
    assert_matches!("myexec.exect".try_into(), Ok(Argument::Path {..}));
    assert_matches!("rm".try_into(), Ok(Argument::Target {..}));
    assert_matches!("-rf".try_into(), Ok(Argument::Alias {..}));
    assert_matches!("--please".try_into(), Ok(Argument::Flag {..}));
    assert_matches!("--opt=3".try_into(), Ok(Argument::Setting {
        value: CliType::Integer {
            value: 3
        },
        ..
    }));
    assert_matches!(TryInto::<Argument<'static>>::try_into("--key=impossible"), Err(
        IoError::UnknownSettingValue { 
            value: "impossible", 
            ..
        }
    ));
    assert_matches!(TryInto::<Argument<'static>>::try_into("&"), Err(
        IoError::FailureParsingArgument {
            argument: "&"
        }
    ));
}

//> TESTS -> READ
#[test]
fn read() -> () {assert_eq!(
    System::path("Cargo.toml").file::<{OpenMode::Read}>(None).unwrap().read_bytes().unwrap().into_iter().next(),
    Some(b'[')
)}