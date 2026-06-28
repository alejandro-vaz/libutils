//^
//^ UTILS
//^

//> UTILS -> ATTRIBUTES
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]
#![feature(default_field_values)]
#![feature(adt_const_params)]
#![feature(unsized_const_params)]
#![feature(transmute_neo)]
#![feature(phantom_variance_markers)]
#![feature(const_default)]
#![feature(const_convert)]
#![feature(const_iter)]
#![feature(const_index)]
#![feature(const_slice_make_iter)]
#![feature(const_try)]
#![feature(allocator_api)]
#![feature(const_closures)]
#![feature(const_option_ops)]

//> UTILS -> ARRAY
#[cfg(feature = "array")]
pub mod array;

//> UTILS -> CAGE
#[cfg(feature = "cage")]
pub mod cage;

//> UTILS -> DIFF
#[cfg(feature = "diff")]
pub mod diff;

//> UTILS -> ISSUE
#[cfg(feature = "issue")]
pub mod issue;

//> UTILS -> LOG
#[cfg(feature = "log")]
pub mod log;

//> UTILS -> POINTER
#[cfg(feature = "pointer")]
pub mod pointer;

//> UTILS -> PROBLEM
#[cfg(feature = "problem")]
pub mod problem;

//> UTILS -> REPORT
#[cfg(feature = "report")]
pub mod report;

//> UTILS -> TERMINAL
#[cfg(feature = "terminal")]
pub mod terminal;