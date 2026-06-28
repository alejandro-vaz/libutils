//^
//^ HEAD
//^

//> HEAD -> MODULES
mod act;
mod shortcut;

//> HEAD -> CRATE
use crate::{
    problem::{
        Threat,
        Severity
    },
    issue::Issue,
    terminal::{
        TERMINAL,
        Console
    },
    array::Array
};

//> HEAD -> ACT
pub use act::Act;

//> HEAD -> CORE
use core::mem::transmute_neo as transmute;


//^
//^ REPORT
//^

//> REPORT -> STRUCT
pub struct Report<const NAME: &'static str, const N: usize> {
    chain: Array<&'static str, N>
}

//> REPORT -> DEFAULT
const impl Default for Report<"Main", 1> {
    fn default() -> Self {
        let mut chain = Array::default();
        chain.push("Main");
        return Self {
            chain: chain
        }
    }
}

//> REPORT -> IMPLEMENTATION
impl<const NAME: &'static str, const N: usize> Report<NAME, N> {
    #[inline]
    pub fn warn<Object: Into<Issue>>(&self, object: Object) -> () {TERMINAL.write().problem(Threat {
        chain: self.chain.clone(),
        object: object,
        severity: Severity::Warning
    })}
    #[inline]
    pub fn error<Object: Into<Issue>>(&self, object: Object) -> () {TERMINAL.write().problem(Threat {
        chain: self.chain.clone(),
        object: object,
        severity: Severity::Error
    })}
    #[inline]
    pub fn sub<'valid, const OTHER: &'static str>(&'valid self) -> Report<OTHER, {N + 1}> {
        let mut chain = Array::from_iter(self.chain.clone().into_iter());
        chain.push(OTHER);
        return Report {
            chain: chain
        }
    }
    #[inline]
    pub fn with<Type>(self, value: Type) -> Act<Type> {return unsafe {transmute(Some(value))}}
    #[inline]
    pub fn with_default<Type: Default>(self) -> Act<Type> {return unsafe {transmute(Some(Type::default()))}}
    #[inline]
    pub fn fail<Type, Object: Into<Issue>>(self, object: Object) -> Act<Type> {
        TERMINAL.write().problem(Threat {
            chain: self.chain.clone(),
            object: object,
            severity: Severity::Critical
        });
        return unsafe {transmute(None::<Type>)};
    }
}