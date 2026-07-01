//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(default_field_values)]
#![feature(const_trait_impl)]

//> HEAD -> MODULES
mod layout;
mod problem;
#[cfg(test)]
mod tests;

//> HEAD -> STD
use std::{
    env::args, 
    io::{
        Write, 
        stderr
    }, 
    sync::LazyLock,
    fs::read_to_string,
    path::PathBuf
};

//> HEAD -> CAGE
use libutils_cage::Cage;

//> HEAD -> PROBLEM
use libutils_threat::Threat;

//> HEAD -> DIFF
use libutils_diff::Diff;

//> HEAD -> LAYOUT
use layout::Layout;

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


//^
//^ TERMINAL
//^

//> TERMINAL -> INSTANCE
pub static TERMINAL: Cage<Terminal> = Cage::new(Terminal {..});

//> TERMINAL -> STRUCT
pub struct Terminal {
    layout: Layout = Layout {..},
    arguments: LazyLock<Vec<Argument>> = LazyLock::new(|| args().map(Argument::from).collect()),
    stderr: String = String::new()
}

//> TERMINAL -> IMPLEMENTATION
impl Console for Terminal {
    #[inline]
    fn arguments(&self) -> &[Argument] {return self.arguments.as_slice()}
    #[inline]
    fn read(&self, filename: &str) -> Result<String, Issue> {return read_to_string(PathBuf::from(filename)).map_err(|error| Issue {
        name: "Failed to read file",
        description: Some(error.to_string()),
        severity: Severity::Error
    })}
    #[inline]
    fn sync(&mut self) -> () {
        let content = self.layout.view();
        stderr().lock().write(<Diff as Into<Vec<u8>>>::into(Diff::new(
            self.stderr.as_bytes(), 
            content.as_bytes())
        ).as_ref()).unwrap();
        stderr().lock().flush().unwrap();
        self.stderr = content;
    }
    #[inline]
    fn problem(&mut self, threat: Threat) -> () {self.layout.problems.push(threat.into())}
}