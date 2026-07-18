//^
//^ HEAD
//^

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> LINTS
#![allow(incomplete_features)]

//> HEAD -> FEATURES
#![feature(const_trait_impl)]
#![feature(const_param_ty_trait)]
#![feature(core_io)]
#![feature(generic_const_exprs)]
#![feature(try_blocks)]
#![feature(min_adt_const_params)]
#![feature(default_field_values)]

//> HEAD -> MODULES
mod argument;
mod clitype;
mod descriptor;
mod ioerror;
mod metadata;
mod openmode;
mod path;
mod problem;

//> HEAD -> STD
use std::{
    env::args,
    sync::LazyLock,
    panic::set_hook,
    path::PathBuf
};

//> HEAD -> CORE
use core::fmt::{
    Debug,
    Display
};

//> HEAD -> IOERROR
pub use ioerror::IoError;

//> HEAD -> ISSUING
use issuing::Issue;

//> HEAD -> PROBLEM
use problem::Problem;

//> HEAD -> ARGUMENT
pub use argument::Argument;

//> HEAD -> CLITYPE
pub use clitype::CliType;

//> HEAD -> RICH_RUST
use rich_rust::console::Console;

//> HEAD -> PATH
use path::Path;

//> HEAD -> OPENMODE
pub use openmode::OpenMode;


//^
//^ SYSTEM
//^

//> SYSTEM -> DATA
static ARGUMENTS: LazyLock<Vec<Argument>> = LazyLock::new(|| {
    return args().map(Argument::try_from).map(|argument| {match argument {
        Ok(value) => value,
        Err(ioerror) => System::critical(ioerror, &["System"])
    }}).collect();
});
static CONSOLE: LazyLock<Console> = LazyLock::new(|| {
    return Console::builder().highlight(false).build();
});

//> SYSTEM -> STRUCT
pub enum System {}

//> SYSTEM -> IMPLEMENTATION
impl System {
    pub fn arguments() -> &'static [Argument] {return ARGUMENTS.as_slice()}
    pub fn path(
        filename: impl Into<PathBuf>
    ) -> Path {return Path {
        name: filename.into()
    }}
    pub fn warning(error: impl Into<Issue>, chain: &[&'static str]) -> () {
        CONSOLE.print(&Problem {
            chain: chain,
            issue: Into::<Issue>::into(error),
            severity: Some(false)
        }.to_string())
    }
    pub fn error(error: impl Into<Issue>, chain: &[&'static str]) -> () {
        CONSOLE.print(&Problem {
            chain: chain,
            issue: Into::<Issue>::into(error),
            severity: Some(true)
        }.to_string())
    }
    pub fn critical(error: impl Into<Issue>, chain: &[&'static str]) -> ! {
        CONSOLE.print(&Problem {
            chain: chain,
            issue: Into::<Issue>::into(error),
            severity: None
        }.to_string());
        set_hook(Box::new(|_| ()));
        panic!();
    }
    pub fn expect<Type>(
        result: Result<Type, impl Into<Issue>>, 
        chain: &[&'static str]
    ) -> Type {return match result {
        Ok(value) => value,
        Err(error) => Self::critical(error, chain)
    }}
    pub fn print(value: impl Display) -> () {CONSOLE.print(&value.to_string())}
    pub fn debug(value: impl Debug, raw: bool) -> () {
        CONSOLE.print(&if raw {format!("{value:?}")} else {format!("{value:#?}")});
    }
}
