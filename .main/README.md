# libutils -- a general utility library

This is a general utility library containing all my crates to remove dependency bloat.

## Crate list

| Crate | Description |
|:-|:-|
| [`active-reporting`](https://crates.io/crates/active-reporting) | A custom error wrapper for detailed logs |
| [`bytediff`](https://crates.io/crates/bytediff) | A bytearray diff |
| [`cagelock`](https://crates.io/crates/cagelock) | A thread-safe static mutable wrapper |
| [`constrangeiter`](https://crates.io/crates/constrangeiter) | Constant iterators for ranges |
| [`ebnftobnf`](https://crates.io/crates/ebnftobnf) | A straightforward EBNF to BNF reducer |
| [`issuing`](https://crates.io/crates/issuing) | A easy-to-use customizable error type |
| [`stack-array`](https://crates.io/crates/stack-array) | A maximum-sized array stored on the stack |
| [`systemio`](https://crates.io/crates/systemio) | A simple generalized I/O interface |
| [`systemstd`](https://crates.io/crates/systemstd) | A custom embedded terminal handler |

## Features

Each crate has one feature which can be enabled or disabled to include or exclude that particular crate. All features are enabled by default.

> Due to this crate making use of many nightly features, the MSRV is 1.99 on edition 2024.