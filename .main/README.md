# libutils -- a general utility library

This is a general utility library containing all my crates to remove dependency bloat.

## Crate list

| Crate | Description |
|:-|:-|
| [`bytediff`](https://crates.io/crates/bytediff) | A bytearray diff |
| [`constrangeiter`](https://crates.io/crates/constrangeiter) | Constant iterators for ranges |
| [`libutils-array`](https://crates.io/crates/libutils-array) | A maximum-sized array stored on the stack |
| [`libutils-cage`](https://crates.io/crates/libutils-cage) | A thread-safe static mutable wrapper |
| [`libutils-console`](https://crates.io/crates/libutils-console) | A simple generalized I/O interface |
| [`libutils-issue`](https://crates.io/crates/libutils-issue) | A easy-to-use customizable error type |
| [`libutils-report`](https://crates.io/crates/libutils-report) | A custom error wrapper for detailed logs |
| [`libutils-terminal`](https://crates.io/crates/libutils-terminal) | A custom embedded terminal handler |

## Features

Each crate has one feature which can be enabled or disabled to include or exclude that particular crate. All features are enabled by default.

> Due to this crate making use of many nightly features, the MSRV is 1.98 on edition 2024.