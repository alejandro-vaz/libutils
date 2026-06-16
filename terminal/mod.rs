//^
//^ HEAD
//^

//> HEAD -> MODULES
mod diff;
mod handler;
mod layout;

//> HEAD -> STD
use std::{
    env::{
        args,
        vars
    },
    collections::HashMap
};

//> HEAD -> HANDLER
use handler::Handler;

//> HEAD -> CORE
use core::default::Default;

//> HEAD -> CRATE
use crate::{
    cage::Cage,
    report::Issue
};

//> HEAD -> DIFF
use diff::Diff;

//> HEAD -> LAYOUT
use layout::Layout;


//^
//^ TERMINAL
//^

//> TERMINAL -> INSTANCE
pub static TERMINAL: Cage<Terminal> = Cage::new(Terminal {
    handler: Handler,
    layout: Layout::default(),
    stderr: String::new()
});

//> TERMINAL -> STRUCT
pub struct Terminal {
    handler: Handler,
    layout: Layout,
    stderr: String
}

//> TERMINAL -> IMPLEMENTATION
impl Terminal {
    pub fn arguments(&self) -> Vec<String> {args().collect()}
    pub fn environment(&self) -> HashMap<String, String> {vars().collect()}
    #[inline]
    fn render(&mut self) -> () {
        let content = self.layout.view();
        self.handler.sync(Diff::new(&self.stderr, &content));
        self.stderr = content;
    }
    #[inline]
    pub fn error(&mut self, issue: Issue) -> () {
        self.layout.logs.push(issue);
        self.render();
    }
}