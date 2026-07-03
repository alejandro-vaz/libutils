//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(const_trait_impl)]
#![feature(const_default)]

//> HEAD -> MODULES
mod item;
mod problem;
#[cfg(test)]
mod tests;

//> HEAD -> STD
use std::{
    env::args, 
    io::{
        Write, 
        stdout
    }, 
    sync::LazyLock,
    fs::read_to_string,
    path::PathBuf
};

//> HEAD -> CORE
use core::fmt::Display;

//> HEAD -> CAGE
use libutils_cage::Cage;

//> HEAD -> PROBLEM
use libutils_threat::Threat;

//> HEAD -> DIFF
use libutils_diff::Diff;

//> HEAD -> ISSUE
use libutils_issue::{
    Issue,
    Severity
};

//> HEAD -> CONSOLE
use libutils_console::{
    Console,
    Argument
};

//> HEAD -> ITEM
use item::Item;


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
    layout: Cage<Vec<Item>>,
    output: Cage<String>
}

//> TERMINAL -> IMPLEMENTATION
impl Console for Terminal {
    #[inline]
    fn arguments<'valid>(&'valid self) -> &'valid [Argument] {return self.arguments.as_slice()}
    #[inline]
    fn read(&self, filename: &str) -> Result<String, Issue> {return read_to_string(PathBuf::from(filename)).map_err(|error| Issue {
        name: "Failed to read file",
        description: Some(error.to_string()),
        severity: Severity::Error
    })}
    #[inline]
    fn sync(&self) -> () {
        let content = self.layout.read(|layout| layout.iter().map(ToString::to_string).collect::<Vec<String>>()).join("\n\n");
        self.output.write(|output| {
            let mut lock = stdout().lock();
            lock.write(<Diff as Into<Vec<u8>>>::into(Diff::new(
                output.as_bytes(), 
                content.as_bytes()
            )).as_ref()).unwrap();
            lock.flush().unwrap();
            *output = content;
        });
    }
    #[inline]
    fn problem(&self, threat: Threat) -> () {return self.layout.write(|layout| layout.push(Item::Problem(threat.into())))}
    #[inline]
    fn print<Type: Display>(&self, value: &Type) -> () {return self.layout.write(|layout| layout.push(Item::String(value.to_string())))}
}