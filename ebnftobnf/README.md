# enbftobnf

This crate has includes a small `reduce` function which converts an EBNF CFG into BNF.

## Problem it solves

Transpiling from EBNF to BNF is an irritating process and writing a reducer that can do so without errors is complex.

## Usage

```rust
#![feature(default_field_values)]

use ebnftobnf::{
    reduce,
    Settings
};

fn main() -> () {
    let ebnf = "A: B+ (C* D)?";
    let bnf: String = reduce(ebnf, Settings {..});
}
```

## BNF format

The input and output format can be customized with the struct `Settings`. It allows for changing many aspects of the (E)BNF flavour such as commenty style, delimiters, how temporal productions look, and whether to include comments or empty lines in the produced EBNF, among others.

## When to use it

Use this crate when you have some EBNF grammar and want to convert it to BNF without much hassle.