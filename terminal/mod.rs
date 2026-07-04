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
    sync::LazyLock,
    fs::File,
    path::PathBuf
};

//> HEAD -> CORE
use core::fmt::{
    Display,
    Debug
};

//> HEAD -> CAGE
use libutils_cage::Cage;

//> HEAD -> PROBLEM
use libutils_threat::Threat;

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
    #[inline]
    fn arguments<'valid>(&'valid self) -> &'valid [Argument] {return self.arguments.as_slice()}
    #[inline]
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
    #[inline]
    fn problem(&self, threat: Threat) -> impl Synchronization {
        self.layout.write(|layout| layout.push(Section::Problem(threat.into())));
        return ActionRequired;
    }
    #[inline]
    fn print(&self, value: impl Display) -> impl Synchronization {
        self.layout.write(|layout| layout.push(Section::Display(value.to_string())));
        return ActionRequired;
    }
    #[inline]
    fn debug(&self, value: impl Debug) -> impl Synchronization {
        self.layout.write(|layout| layout.push(Section::Debug(format!("{value:#?}"))));
        return ActionRequired;
    }
}