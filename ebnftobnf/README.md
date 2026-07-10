# enbftobnf

This crate has includes a small `reduce` function which converts an EBNF CFG into BNF.

## Problem it solves

Transpiling from EBNF to BNF is an irritating process and writing a reducer that can do so without errors is complex.

## Usage

```rust
use libutils::ebnftobnf::reduce;

fn main() -> () {
    let ebnf = "A: B+ (C* D)?";
    let bnf: String = reduce(ebnf);
}
```

## BNF format

The function returns a `String` who has deduplicated temporal productions. The format of a temporal production is `${N}` where N is a number.

The delimiter between the rule name and the productions and derivations is now fixed to be `": "`.

## When to use it

Use this crate when you have some EBNF grammar and want to convert it to BNF without much hassle.