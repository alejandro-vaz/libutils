//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(default_field_values)]

//> HEAD -> MODULES
mod console;
mod layout;
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
    },
    time::Instant
};

//> HEAD -> HASHBROWN
use hashbrown::HashMap as Map;

//> HEAD -> CAGE
use cage::Cage;

//> HEAD -> PROBLEM
use problem::{
    Problem,
    Threat,
    Threaten
};

//> HEAD -> DIFF
use diff::Diff;

//> HEAD -> ISSUE
use issue::Issue;

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
    layout: Layout,
    stderr: String
}

//> TERMINAL -> THREATEN
impl Threaten for Terminal {
    #[inline]
    fn convert<Object: Into<Issue>, const N: usize>(&mut self, threat: Threat<Object, N>) -> Problem {return Problem {
        chain: threat.chain.into(),
        at: Instant::now(),
        issue: threat.object.into(),
        severity: threat.severity
    }}
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
    fn problem<Object: Into<Issue>, const N: usize>(&mut self, threat: Threat<Object, N>) -> () {
        let problem = self.convert(threat);
        self.layout.problems.push(problem);
        self.sync();
    }
}