//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![allow(incomplete_features)]
#![feature(transmute_neo)]
#![feature(const_destruct)]
#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]
#![feature(const_try)]
#![feature(const_default)]
#![feature(const_trait_impl)]
#![feature(unsized_const_params)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_try_residual)]
#![feature(const_option_ops)]

//> HEAD -> MODULES
mod act;
mod shortcut;
#[cfg(test)]
mod tests;

//> HEAD -> ARRAY
use libutils_array::Array;

//> HEAD -> TERMINAL
use libutils_terminal::{
    TERMINAL,
    Console
};

//> HEAD -> ISSUE
use libutils_issue::Issue;

//> HEAD -> PROBLEM
use libutils_problem::{
    Threat,
    Severity
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