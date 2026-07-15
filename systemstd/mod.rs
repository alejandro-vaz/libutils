//^
//^ HEAD
//^

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> FEATURES
#![feature(const_trait_impl)]
#![feature(const_default)]
#![feature(core_io)]
#![feature(transmute_neo)]

//> HEAD -> MODULES
mod argument;
mod clitype;
mod descriptor;
mod ioerror;
mod metadata;
mod problem;
mod section;
mod update;

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

//> HEAD -> LOCKS
use locks::Mutex;

//> HEAD -> IOERROR
pub use ioerror::IoError;

//> HEAD -> UPDATE
use update::Update;

//> HEAD -> ISSUING
use issuing::Issue;

//> HEAD -> SECTION
use section::Section;

//> HEAD -> PROBLEM
use problem::Problem;

//> HEAD -> DESCRIPTOR
use descriptor::Descriptor;

//> HEAD -> ARGUMENT
pub use argument::Argument;

//> HEAD -> CLITYPE
pub use clitype::CliType;


//^
//^ SYSTEM
//^

//> SYSTEM -> DATA
static ARGUMENTS: LazyLock<Vec<Argument>> = LazyLock::new(|| args().map(Argument::from).collect());
static LAYOUT: Mutex<Vec<Section>> = Mutex::default();
static OUTPUT: Mutex<String> = Mutex::default();

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
    pub fn warning(error: impl Into<Issue>, chain: &[&'static str]) -> Update {
        LAYOUT.with(|layout| layout.push(Section::Problem(Problem {
            chain: Vec::from(chain),
            issue: Into::<Issue>::into(error),
            severity: Some(false),
            _at: Instant::now()
        })));
        return Update;
    }
    pub fn error(error: impl Into<Issue>, chain: &[&'static str]) -> Update {
        LAYOUT.with(|layout| layout.push(Section::Problem(Problem {
            chain: Vec::from(chain),
            issue: Into::<Issue>::into(error),
            severity: Some(true),
            _at: Instant::now()
        })));
        return Update;
    }
    pub fn critical(error: impl Into<Issue>, chain: &[&'static str]) -> ! {
        LAYOUT.with(|layout| layout.push(Section::Problem(Problem {
            chain: Vec::from(chain),
            issue: Into::<Issue>::into(error),
            severity: None,
            _at: Instant::now()
        })));
        Update.sync();
        set_hook(Box::new(|_| ()));
        panic!();
    }
    pub fn expect<Type>(result: Result<Type, impl Into<Issue>>, chain: &[&'static str]) -> Type {
        return match result {
            Ok(value) => value,
            Err(error) => Self::critical(error, chain)
        }
    }
    pub fn print(value: impl Display) -> Update {
        LAYOUT.with(|layout| layout.push(Section::Display(value.to_string())));
        return Update;
    }
    pub fn debug(value: impl Debug) -> Update {
        LAYOUT.with(|layout| layout.push(Section::Debug(format!("{value:#?}"))));
        return Update;
    }
    pub fn clear() -> Update {
        LAYOUT.with(Vec::clear);
        return Update;
    }
}
