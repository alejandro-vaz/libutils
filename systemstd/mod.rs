//^
//^ HEAD
//^

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> FEATURES
#![feature(const_trait_impl)]
#![feature(const_default)]
#![feature(test)]
#![feature(transmute_neo)]

//> HEAD -> CRATES
extern crate test;

//> HEAD -> MODULES
#[cfg(test)]
mod benches;
mod descriptor;
mod metadata;
mod problem;
mod section;
#[cfg(test)]
mod tests;
mod update;

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

//> HEAD -> CAGELOCK
use cagelock::Cage;

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
static LAYOUT: Cage<Vec<Section>> = Cage::default();
static OUTPUT: Cage<String> = Cage::default();

//> SYSTEM -> STRUCT
pub struct System;

//> SYSTEM -> IMPLEMENTATION
impl SystemIO for System {
    fn arguments() -> &'static [Argument] {return ARGUMENTS.as_slice()}
    fn open(filename: &str) -> Result<impl DescriptorTrait, Issue> {
        match File::open(PathBuf::from(filename)) {
            Ok(file) => Ok(Descriptor {
                file: file
            }),
            Err(error) => Err(Issue {
                name: "Failed to open file",
                description: Some(error.to_string()),
                severity: Severity::Error
            })
        }
    }
    fn problem(issue: Issue, chain: &[&'static str]) -> impl UpdateTrait {
        LAYOUT.write(|layout| layout.push(Section::Problem(Problem {
            chain: Vec::from(chain),
            issue: issue,
            _at: Instant::now()
        })));
        return Update;
    }
    fn print(value: impl Display) -> impl UpdateTrait {
        LAYOUT.write(|layout| layout.push(Section::Display(value.to_string())));
        return Update;
    }
    fn debug(value: impl Debug) -> impl UpdateTrait {
        LAYOUT.write(|layout| layout.push(Section::Debug(format!("{value:#?}"))));
        return Update;
    }
    fn clear() -> impl UpdateTrait {
        LAYOUT.write(Vec::clear);
        return Update;
    }
}