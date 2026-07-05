//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(const_trait_impl)]
#![feature(const_default)]

//> HEAD -> MODULES
mod continuations;
mod problem;
mod section;
#[cfg(test)]
mod tests;

//> HEAD -> STD
use std::{
    env::args, 
    fs::File, 
    path::PathBuf, 
    sync::LazyLock, 
    time::Instant
};

//> HEAD -> CORE
use core::fmt::{
    Debug, 
    Display
};

//> HEAD -> CAGE
use libutils_cage::Cage;

//> HEAD -> CONTINUATIONS
use continuations::{
    ActionRequired,
    FileDescriptor
};

//> HEAD -> ISSUE
use libutils_issue::{
    Issue,
    Severity
};

//> HEAD -> CONSOLE
use libutils_console::{
    Console,
    Argument,
    Synchronization,
    Descriptor
};

//> HEAD -> SECTION
use section::Section;

//> HEAD -> PROBLEM
use problem::Problem;


//^
//^ TERMINAL
//^

//> TERMINAL -> INSTANCE
pub static TERMINAL: Terminal = Terminal {
    arguments: LazyLock::new(|| args().map(Argument::from).collect()),
    layout: Cage::default(),
    output: Cage::default()
};

//> TERMINAL -> STRUCT
pub struct Terminal {
    arguments: LazyLock<Vec<Argument>>,
    layout: Cage<Vec<Section>>,
    output: Cage<String>
}

//> TERMINAL -> IMPLEMENTATION
impl Console for Terminal {
    fn arguments<'valid>(&'valid self) -> &'valid [Argument] {return self.arguments.as_slice()}
    fn open(&self, filename: &str) -> Result<impl Descriptor, Issue> {match File::open(PathBuf::from(filename)) {
        Ok(file) => Ok(FileDescriptor {
            file: file
        }),
        Err(error) => Err(Issue {
            name: "Failed to open file",
            description: Some(error.to_string()),
            severity: Severity::Error
        })
    }}
    fn problem(&self, issue: Issue, chain: &[&'static str]) -> impl Synchronization {
        self.layout.write(|layout| layout.push(Section::Problem(Problem {
            chain: Vec::from(chain),
            issue: issue,
            _at: Instant::now()
        })));
        return ActionRequired;
    }
    fn print(&self, value: impl Display) -> impl Synchronization {
        self.layout.write(|layout| layout.push(Section::Display(value.to_string())));
        return ActionRequired;
    }
    fn debug(&self, value: impl Debug) -> impl Synchronization {
        self.layout.write(|layout| layout.push(Section::Debug(format!("{value:#?}"))));
        return ActionRequired;
    }
}