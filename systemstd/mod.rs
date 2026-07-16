//^
//^ HEAD
//^

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> FEATURES
#![feature(const_trait_impl)]
#![feature(core_io)]
#![feature(transmute_neo)]

//> HEAD -> MODULES
mod argument;
mod clitype;
mod descriptor;
mod ioerror;
mod metadata;
mod problem;

//> HEAD -> STD
use std::{
    env::args,
    fs::File,
    path::PathBuf,
    sync::LazyLock,
    time::Instant,
    panic::set_hook
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

//> HEAD -> DESCRIPTOR
use descriptor::Descriptor;

//> HEAD -> ARGUMENT
pub use argument::Argument;

//> HEAD -> CLITYPE
pub use clitype::CliType;

//> HEAD -> RICH_RUST
use rich_rust::Console;


//^
//^ SYSTEM
//^

//> SYSTEM -> DATA
static ARGUMENTS: LazyLock<Vec<Argument>> = LazyLock::new(|| args().map(Argument::from).collect());
static CONSOLE: LazyLock<Console> = LazyLock::new(|| Console::new());

//> SYSTEM -> STRUCT
pub enum System {}

//> SYSTEM -> IMPLEMENTATION
impl System {
    pub fn arguments() -> &'static [Argument] {return ARGUMENTS.as_slice()}
    pub fn open(filename: &str) -> Result<Descriptor, IoError> {match File::open(PathBuf::from(filename)) {
        Ok(file) => Ok(Descriptor {
            file: file
        }),
        Err(error) => Err(IoError::CouldntOpenFile {error})
    }}
    pub fn warning(error: impl Into<Issue>, chain: &[&'static str]) -> () {CONSOLE.print(&Problem {
        chain: Vec::from(chain),
        issue: Into::<Issue>::into(error),
        severity: Some(false),
        _at: Instant::now()
    }.to_string())}
    pub fn error(error: impl Into<Issue>, chain: &[&'static str]) -> () {CONSOLE.print(&Problem {
        chain: Vec::from(chain),
        issue: Into::<Issue>::into(error),
        severity: Some(true),
        _at: Instant::now()
    }.to_string())}
    pub fn critical(error: impl Into<Issue>, chain: &[&'static str]) -> ! {
        CONSOLE.print(&Problem {
            chain: Vec::from(chain),
            issue: Into::<Issue>::into(error),
            severity: None,
            _at: Instant::now()
        }.to_string());
        set_hook(Box::new(|_| ()));
        panic!();
    }
    pub fn expect<Type>(result: Result<Type, impl Into<Issue>>, chain: &[&'static str]) -> Type {return match result {
        Ok(value) => value,
        Err(error) => Self::critical(error, chain)
    }}
    pub fn print(value: impl Display) -> () {CONSOLE.print(&value.to_string())}
    pub fn debug(value: impl Debug) -> () {CONSOLE.print(&format!("{value:#?}"))}
    pub fn debug_raw(value: impl Debug) -> () {CONSOLE.print(&format!("{value:?}"))}
}
