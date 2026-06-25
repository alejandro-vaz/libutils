//^
//^ HEAD
//^

//> HEAD -> MODULES
mod console;
mod layout;

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
    issue::Issue,
    problem::Problem,
    diff::Diff
};

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

//> TERMINAL -> IMPLEMENTATION
impl Console for Terminal {
    fn arguments(&self) -> Vec<String> {args().collect()}
    fn environment(&self) -> Map<String, String> {vars().collect()}
    #[inline]
    fn render(&mut self) -> () {
        let content = self.layout.view();
        stderr().lock().write(<Diff as Into<Vec<u8>>>::into(Diff::new(self.stderr.as_bytes(), content.as_bytes())).as_ref()).unwrap();
        self.stderr = content;
    }
    #[inline]
    fn issue<Object: Into<Issue>>(&mut self, problem: Problem<Object>) -> () {
        self.layout.problems.push(Problem {
            chain: problem.chain,
            at: problem.at,
            object: problem.object.into(),
            severity: problem.severity
        });
        self.render();
    }
}