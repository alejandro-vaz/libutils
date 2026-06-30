//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(default_field_values)]

//> HEAD -> MODULES
mod console;
mod layout;
mod problem;
#[cfg(test)]
mod tests;

//> HEAD -> STD
use std::{
    env::{
        args,
        vars
    },
    io::{
        stderr,
        Write
    }
};

//> HEAD -> HASHBROWN
use hashbrown::HashMap as Map;

//> HEAD -> CAGE
use libutils_cage::Cage;

//> HEAD -> PROBLEM
use libutils_threat::Threat;

//> HEAD -> DIFF
use libutils_diff::Diff;

//> HEAD -> ISSUE
use libutils_issue::Issue;

//> HEAD -> LAYOUT
use layout::Layout;

//> HEAD -> CONSOLE
pub use console::Console;


//^
//^ TERMINAL
//^

//> TERMINAL -> INSTANCE
pub static TERMINAL: Cage<Terminal> = Cage::new(Terminal {
    layout: Layout {..},
    stderr: String::new()
});

//> TERMINAL -> STRUCT
pub struct Terminal {
    pub layout: Layout,
    stderr: String
}

//> TERMINAL -> IMPLEMENTATION
impl Console for Terminal {
    #[inline]
    fn arguments(&self) -> Vec<String> {args().collect()}
    #[inline]
    fn environment(&self) -> Map<String, String> {vars().collect()}
    #[inline]
    fn sync(&mut self) -> () {
        let content = self.layout.view();
        stderr().lock().write(<Diff as Into<Vec<u8>>>::into(Diff::new(self.stderr.as_bytes(), content.as_bytes())).as_ref()).unwrap();
        self.stderr = content;
    }
    #[inline]
    fn problem<Object: Into<Issue>>(&mut self, threat: Threat<Object>) -> () {
        self.layout.problems.push(threat.into());
        self.sync();
    }
}