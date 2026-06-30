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
#![feature(generic_const_exprs)]

//> HEAD -> MODULES
mod state;
#[cfg(test)]
mod tests;

//> HEAD -> PUBLIC STATE
pub use state::{
    Add,
    Stay
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
use libutils_terminal::{
    TERMINAL,
    Console
};

//> HEAD -> THREAT
use libutils_threat::{
    Threat,
    Severity
};


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
        Self {
            data: Main {
                chain: chain
            }
        }
    }
}

//> REPORT -> GIVE
impl<Current: State> Report<Current> {
    #[inline]
    pub fn to<'valid, Following: DerivedState<'valid>>(&'valid mut self) -> Report<Following> {return Report {
        data: Following::from(&mut self.data)
    }}
    #[inline]
    pub fn warn<Object: Into<Issue>, Wants: Default>(&self, object: Object) -> Wants {
        TERMINAL.write().problem(Threat {
            object: object,
            chain: self.data.chain(),
            severity: Severity::Warning
        });
        return Wants::default();
    }
    #[inline]
    pub fn error<Object: Into<Issue>, Wants: Default>(&self, object: Object) -> Wants {
        TERMINAL.write().problem(Threat {
            object: object,
            chain: self.data.chain(),
            severity: Severity::Error
        });
        return Wants::default();
    }
    #[inline]
    pub fn critical<Object: Into<Issue>, Wants: Default>(&self, object: Object) -> Wants {
        TERMINAL.write().problem(Threat {
            object: object,
            chain: self.data.chain(),
            severity: Severity::Critical
        });
        return Wants::default();
    }
}