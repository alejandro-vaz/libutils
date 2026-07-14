//^
//^ HEAD
//^

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> FEATURES
#![feature(const_trait_impl)]
#![feature(const_default)]
#![feature(default_field_values)]
#![feature(transmute_neo)]

//> HEAD -> MODULES
mod descriptor;
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

//> HEAD -> UPDATE
use update::Update;

//> HEAD -> ISSUING
use issuing::{
    Issue,
    Severity
};

//> HEAD -> SYSTEMIO
use systemio::{
    SystemIO,
    Argument,
    Update as UpdateTrait,
    Descriptor as DescriptorTrait
};

//> HEAD -> SECTION
use section::Section;

//> HEAD -> PROBLEM
use problem::Problem;

//> HEAD -> DESCRIPTOR
use descriptor::Descriptor;


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
impl SystemIO for System {
    fn arguments() -> &'static [Argument] {return ARGUMENTS.as_slice()}
    fn open(filename: &str) -> Result<impl DescriptorTrait, Issue> {match File::open(PathBuf::from(filename)) {
        Ok(file) => Ok(Descriptor {
            file: file
        }),
        Err(error) => Err(Issue {
            name: "Failed to open file",
            description: Some(error.to_string()),
            ..
        })
    }}
    fn problem(error: impl Into<Issue>, chain: &[&'static str]) -> impl UpdateTrait {
        let issue = Into::<Issue>::into(error);
        let critical = if let Severity::Critical = issue.severity {true} else {false};
        LAYOUT.get(|layout| layout.push(Section::Problem(Problem {
            chain: Vec::from(chain),
            issue: issue,
            _at: Instant::now()
        })));
        return if critical {
            Update.sync();
            set_hook(Box::new(|_| ()));
            panic!();
        } else {
            Update
        }
    }
    fn print(value: impl Display) -> impl UpdateTrait {
        LAYOUT.get(|layout| layout.push(Section::Display(value.to_string())));
        return Update;
    }
    fn debug(value: impl Debug) -> impl UpdateTrait {
        LAYOUT.get(|layout| layout.push(Section::Debug(format!("{value:#?}"))));
        return Update;
    }
    fn clear() -> impl UpdateTrait {
        LAYOUT.get(Vec::clear);
        return Update;
    }
}