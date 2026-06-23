//^
//^ HEAD
//^

//> HEAD -> MODULES
mod console;
mod diff;
mod layout;
mod problem;

//> HEAD -> STD
use std::{
    env::{
        args,
        vars
    },
    collections::HashMap as Map,
    io::{
        stderr,
        Write
    }
};

//> HEAD -> CRATE
use crate::{
    cage::Cage,
    report::ToIssue
};

//> HEAD -> DIFF
use diff::Diff;

//> HEAD -> LAYOUT
use layout::Layout;

//> HEAD -> PROBLEM
pub use problem::{
    Problem,
    Severity
};

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

//> TERMINAL -> IMPLEMENTATION
impl Console for Terminal {
    fn arguments(&self) -> Vec<String> {args().collect()}
    fn environment(&self) -> Map<String, String> {vars().collect()}
    #[inline]
    fn render(&mut self) -> () {
        let content = self.layout.view();
        stderr().lock().write(<Diff as Into<Vec<u8>>>::into(Diff::new(&self.stderr, &content)).as_ref()).unwrap();
        self.stderr = content;
    }
    #[inline]
    fn issue<Object: ToIssue>(&mut self, problem: &Problem<Object>) -> () {
        self.layout.problems.push(Problem {
            chain: problem.chain.clone(),
            at: problem.at,
            object: problem.object.to_issue(),
            severity: problem.severity
        });
        self.render();
    }
}