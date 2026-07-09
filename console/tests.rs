//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Argument;

//> HEAD -> ALLOC
use alloc::string::ToString;

//> HEAD -> CORE
use core::assert_matches;


//^
//^ TESTS
//^

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