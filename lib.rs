//^
//^ UTILS
//^

//> UTILS -> ATTRIBUTES
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(const_default)]
#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]

//> UTILS -> ARRAY
#[cfg(feature = "array")]
pub mod array;

//> UTILS -> CAGE
#[cfg(feature = "cage")]
pub mod cage;

//> UTILS -> REPORT
#[cfg(feature = "report")]
pub mod report;

//> UTILS -> TERMINAL
#[cfg(feature = "terminal")]
pub mod terminal;