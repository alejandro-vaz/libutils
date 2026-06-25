//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::{
    terminal::{
        TERMINAL,
        Console
    },
    problem::Problem,
    issue::Issue
};


//^
//^ MODE
//^

//> MODE -> TYPE MACHINE
pub trait Mode {
    const NAME: &'static str;
    fn send<Object: Into<Issue>>(&self, problem: Problem<Object>) -> ();
    fn connect<'allowed, const OTHER: &'static str>(&'allowed self) -> Set<'allowed, OTHER>;
}

//> MODE -> MAIN
pub struct Main; impl Mode for Main {
    const NAME: &'static str = "Main";
    fn send<Object: Into<Issue>>(&self, mut problem: Problem<Object>) -> () {
        problem.chain.push(Self::NAME);
        TERMINAL.write().issue(problem)
    }
    fn connect<'allowed, const OTHER: &'static str>(&'allowed self) -> Set<'allowed, OTHER> {return Set(self)}
}

//> MODE -> SET
pub struct Set<'valid, const NAME: &'static str>(&'valid Main);
impl<'valid, const NAME: &'static str> Mode for Set<'valid, NAME> {
    const NAME: &'static str = NAME;
    fn send<Object: Into<Issue>>(&self, mut problem: Problem<Object>) -> () {
        problem.chain.push(Self::NAME);
        self.0.send(problem);
    }
    fn connect<'allowed, const OTHER: &'static str>(&'allowed self) -> Set<'allowed, OTHER> {return Set(self.0)}
}