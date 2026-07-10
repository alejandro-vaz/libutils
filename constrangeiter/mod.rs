//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> LINTS
#![allow(unused_features)]

//> HEAD -> FEATURES
#![feature(const_trait_impl)]
#![feature(const_iter)]
#![feature(const_ops)]
#![feature(const_closures)]
#![feature(const_option_ops)]
#![feature(const_convert)]
#![feature(const_try)]
#![feature(const_cmp)]
#![feature(trusted_len)]
#![feature(test)]
#![feature(const_destruct)]
#![feature(new_range)]

//> HEAD -> CRATES
extern crate alloc;
extern crate test;

//> HEAD -> MODULES
mod range;
mod rangefrom;
mod rangeinclusive;
#[cfg(test)]
mod tests;

//> HEAD -> CORE
use core::{
    ops::{
        AddAssign, 
        SubAssign,
        Sub
    }, 
    marker::Destruct
};


//^
//^ TRAIT
//^

//> TRAIT -> DEFINITION
pub const trait ConstIntoIterator {
    type Item;
    type IntoIter: const Iterator<Item = Self::Item>;
    fn const_into_iter(self) -> Self::IntoIter;
}


//^
//^ TARGET
//^

//> TARGET -> TRAIT
pub const trait Target: Copy + const AddAssign + const SubAssign + const PartialOrd + const Sub<Output = Self> + const Destruct
where usize: const TryFrom<Self>, <usize as TryFrom<Self>>::Error: const Destruct {
    const ONE: Self;
    const MAX: Self;
    fn hint(self, start: Self) -> (usize, Option<usize>) {match usize::try_from(self - start) {
        Ok(number) => (number, Some(number)),
        Err(_) => (usize::MAX, None)
    }}
}

//> TARGET -> U8
const impl Target for u8 {
    const ONE: Self = 1u8;
    const MAX: Self = u8::MAX;
    fn hint(self, start: Self) -> (usize, Option<usize>) {match usize::try_from(self - start) {
        Ok(number) => (number, Some(number)),
        Err(_) => (usize::MAX, None)
    }}
}

//> TARGET -> I8
const impl Target for i8 {
    const ONE: Self = 1i8;
    const MAX: Self = i8::MAX;
    fn hint(self, start: Self) -> (usize, Option<usize>) {match usize::try_from(self - start) {
        Ok(number) => (number, Some(number)),
        Err(_) => (usize::MAX, None)
    }}
}

//> TARGET -> U16
const impl Target for u16 {
    const ONE: Self = 1u16;
    const MAX: Self = u16::MAX;
    fn hint(self, start: Self) -> (usize, Option<usize>) {match usize::try_from(self - start) {
        Ok(number) => (number, Some(number)),
        Err(_) => (usize::MAX, None)
    }}
}

//> TARGET -> I16
const impl Target for i16 {
    const ONE: Self = 1i16;
    const MAX: Self = i16::MAX;
    fn hint(self, start: Self) -> (usize, Option<usize>) {match usize::try_from(self - start) {
        Ok(number) => (number, Some(number)),
        Err(_) => (usize::MAX, None)
    }}
}

//> TARGET -> U32
const impl Target for u32 {
    const ONE: Self = 1u32;
    const MAX: Self = u32::MAX;
    fn hint(self, start: Self) -> (usize, Option<usize>) {match usize::try_from(self - start) {
        Ok(number) => (number, Some(number)),
        Err(_) => (usize::MAX, None)
    }}
}

//> TARGET -> I32
const impl Target for i32 {
    const ONE: Self = 1i32;
    const MAX: Self = i32::MAX;
    fn hint(self, start: Self) -> (usize, Option<usize>) {match usize::try_from(self - start) {
        Ok(number) => (number, Some(number)),
        Err(_) => (usize::MAX, None)
    }}
}

//> TARGET -> U64
const impl Target for u64 {
    const ONE: Self = 1u64;
    const MAX: Self = u64::MAX;
    fn hint(self, start: Self) -> (usize, Option<usize>) {match usize::try_from(self - start) {
        Ok(number) => (number, Some(number)),
        Err(_) => (usize::MAX, None)
    }}
}

//> TARGET -> I64
const impl Target for i64 {
    const ONE: Self = 1i64;
    const MAX: Self = i64::MAX;
    fn hint(self, start: Self) -> (usize, Option<usize>) {match usize::try_from(self - start) {
        Ok(number) => (number, Some(number)),
        Err(_) => (usize::MAX, None)
    }}
}

//> TARGET -> U128
const impl Target for u128 {
    const ONE: Self = 1u128;
    const MAX: Self = u128::MAX;
    fn hint(self, start: Self) -> (usize, Option<usize>) {match usize::try_from(self - start) {
        Ok(number) => (number, Some(number)),
        Err(_) => (usize::MAX, None)
    }}
}

//> TARGET -> I128
const impl Target for i128 {
    const ONE: Self = 1i128;
    const MAX: Self = i128::MAX;
    fn hint(self, start: Self) -> (usize, Option<usize>) {match usize::try_from(self - start) {
        Ok(number) => (number, Some(number)),
        Err(_) => (usize::MAX, None)
    }}
}

//> TARGET -> USIZE
const impl Target for usize {
    const ONE: Self = 1usize;
    const MAX: Self = usize::MAX;
    fn hint(self, start: Self) -> (usize, Option<usize>) {match usize::try_from(self - start) {
        Ok(number) => (number, Some(number)),
        Err(_) => (usize::MAX, None)
    }}
}

//> TARGET -> ISIZE
const impl Target for isize {
    const ONE: Self = 1isize;
    const MAX: Self = isize::MAX;
    fn hint(self, start: Self) -> (usize, Option<usize>) {match usize::try_from(self - start) {
        Ok(number) => (number, Some(number)),
        Err(_) => (usize::MAX, None)
    }}
}