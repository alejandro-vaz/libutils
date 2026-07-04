//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![allow(incomplete_features)]
#![feature(const_trait_impl)]
#![feature(unsized_const_params)]
#![feature(adt_const_params)]
#![feature(default_field_values)]
#![feature(const_heap)]
#![feature(never_type)]
#![feature(const_default)]
#![feature(generic_const_exprs)]

//> HEAD -> MODULES
mod state;
#[cfg(test)]
mod tests;
mod then;

//> HEAD -> PUBLIC STATE
pub use state::{
    Same,
    Name
};

//> HEAD -> STATE
use state::{
    State,
    Main,
    DerivedState
};

//> HEAD -> ISSUE
use libutils_issue::Issue;

//> HEAD -> TERMINAL
use libutils_terminal::TERMINAL;

//> HEAD -> THREAT
use libutils_threat::Threat;

//> HEAD -> CONSOLE
use libutils_console::{
    Console, 
    Synchronization
};

//> HEAD -> THEN
use then::Then;


//^ 
//^ REPORT
//^ 

//> REPORT -> STRUCT
pub struct Report<Current: State> {
    data: Current
}

//> REPORT -> IMPLEMENTATION
impl Report<Main> {
    pub const fn new(name: &'static str) -> Self {
        let mut chain = Vec::new();
        chain.push(name);
        return Self {
            data: Main {
                chain: chain
            }
        };
    }
}

//> REPORT -> DEFAULT
const impl Default for Report<Main> {
    fn default() -> Self {return Self::new("Main")}
}

//> REPORT -> IMPLEMENTATION
impl<Current: State> Report<Current> {
    #[inline]
    pub fn to<'valid, Following: DerivedState<'valid>>(&'valid mut self) -> Report<Following> {return Report {
        data: Following::convert(&mut self.data)
    }}
    #[inline]
    pub fn issue(&self, object: impl Into<Issue>) -> Then<!> {
        TERMINAL.problem(Threat {
            issue: object.into(),
            chain: self.data.chain()
        }).sync();
        return Then {
            value: None
        };
    }
    #[inline]
    pub fn apply<Type>(&self, result: Result<Type, impl Into<Issue>>) -> Then<Type> {return Then {
        value: match result {
            Ok(value) => Some(value),
            Err(issue) => self.issue(issue).none()
        }
    }}
}