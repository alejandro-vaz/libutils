# libutils -- a general utility library

This library is a re-export of all `libutils-*` crates together

## Crate list

| `libutils-{}` | Description |
|:-|:-|
| [`array`](https://crates.io/crates/libutils-array) | A maximum-sized array stored on the stack |
| [`cage`](https://crates.io/crates/libutils-cage) | A thread-safe static mutable wrapper |
| [`console`](https://crates.io/crates/libutils-console) | A simple generalized I/O interface |
| [`constrangeiter`](https://crates.io/crates/libutils-constrangeiter) | Constant iterators for ranges |
| [`diff`](https://crates.io/crates/libutils-constrangeiter) | A bytearray diff |
| [`issue`](https://crates.io/crates/libutils-issue) | A easy-to-use customizable error type |
| [`report`](https://crates.io/crates/libutils-report) | A custom error wrapper for detailed logs |
| [`terminal`](https://crates.io/crates/libutils-terminal) | A custom embedded terminal handler |

## Features

Each crate has one feature which can be enabled or disabled to include or exclude that particular crate. All features are enabled by default.

> Due to this crate making use of many nightly features, the minimum supported version for Rust is 1.98 on edition 2024.