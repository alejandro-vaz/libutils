//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(default_field_values)]
#![feature(const_trait_impl)]

//> HEAD -> MODULES
mod argument;
mod console;
mod layout;
mod problem;
#[cfg(test)]
mod tests;

//> HEAD -> STD
use std::{
    collections::HashMap as Map, 
    env::{
        args,
        vars
    }, 
    io::{
        Write, 
        stderr
    }, 
    sync::LazyLock
};

//> HEAD -> CAGE
use libutils_cage::Cage;

//> HEAD -> PROBLEM
use libutils_threat::Threat;

//> HEAD -> DIFF
use libutils_diff::Diff;

//> HEAD -> LAYOUT
use layout::Layout;

//> HEAD -> CONSOLE
pub use console::Console;

//> HEAD -> ARGUMENT
pub use argument::Argument;


//^
//^ TERMINAL
//^

//> TERMINAL -> INSTANCE
pub static TERMINAL: Cage<Terminal> = Cage::new(Terminal {..});

//> TERMINAL -> STRUCT
pub struct Terminal {
    layout: Layout = Layout {..},
    arguments: LazyLock<Vec<Argument>> = LazyLock::new(|| args().map(Argument::from).collect()),
    pub environment: LazyLock<Map<String, String>> = LazyLock::new(|| vars().collect()),
    stderr: String = String::new()
}

//> TERMINAL -> IMPLEMENTATION
impl Console for Terminal {
    #[inline]
    fn arguments(&self) -> &[Argument] {return self.arguments.as_slice()}
    #[inline]
    fn sync(&mut self) -> () {
        let content = self.layout.view();
        stderr().lock().write(<Diff as Into<Vec<u8>>>::into(Diff::new(self.stderr.as_bytes(), content.as_bytes())).as_ref()).unwrap();
        self.stderr = content;
    }
    #[inline]
    fn problem(&mut self, threat: Threat) -> () {self.layout.problems.push(threat.into())}
}