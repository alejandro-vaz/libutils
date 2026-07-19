//^
//^ HEAD
//^

//> HEAD -> STD
use std::{
    sync::LazyLock,
    env::args
};

//> HEAD -> SUPER
use super::{
    System,
    argument::Argument
};

//> HEAD -> RICH_RUST
use rich_rust::console::Console;


//^ 
//^ DATA
//^ 

//> DATA -> ARGUMENTS
pub static ARGUMENTS: LazyLock<Vec<Argument>> = LazyLock::new(|| {
    return args().map(Argument::try_from).map(|argument| {match argument {
        Ok(value) => value,
        Err(ioerror) => System::critical(ioerror, &["System"])
    }}).collect();
});

//> DATA -> CONSOLE
pub static CONSOLE: LazyLock<Console> = LazyLock::new(|| {
    return Console::builder().highlight(false).build();
});