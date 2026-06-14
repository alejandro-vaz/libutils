//^
//^ UTILS
//^

//> UTILS -> ATTRIBUTES
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]

//> UTILS -> ARRAY
#[cfg(feature = "array")]
pub mod array;

//> UTILS -> CAGE
#[cfg(feature = "cage")]
pub mod cage;

//> UTILS -> REPORT
#[cfg(feature = "report")]
pub mod report;