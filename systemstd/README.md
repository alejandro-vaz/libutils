# Terminal

This is an implementation using the standard library of `Console`.

## Usage

This crate exposes the Zero-Sized Type `Terminal` that just refers to the internals of the implementation.

```rust
use libutils_terminal::Terminal;
use libutils_console::{Console, Update};

let terminal: Terminal = Terminal;

Terminal.print("hello!").sync();
```

For more information about usage and features, see [`libutils-console`](https://crates.io/crates/libutils-console).

## When to use it

Use this type when you want to have a `Console` environment in a normal Rust program.

> ![WARNING] Libraries should **NOT** implement direct calls to a certain `Console` instances like `Terminal`, but allow the caller binary to provide one or return `Result<T, Issue>`.