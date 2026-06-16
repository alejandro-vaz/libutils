# libutils -- general utility library

This library is meant to have types that may be useful for coding executables and libraries that all need shared functionality.

| Module | Description | Use cases | Requires `std` |
|:-|:-|:-|:-|
| [`array`](./tests/array.rs) | A maximum-sized array stored on the stack | Upper-bounded length arrays, unknown amount of elements, speed | No |
| [`cage`](./tests/cage.rs) | A thread-safe static mutable wrapper | Allocator APIs, safe mutable statics, atomic types | Yes |
| [`report`](./tests/report.rs) | Comprehensive error wrappers for detailed logs | Compilers, error chains, logs | Yes |
| [`terminal`](./tests/terminal.rs) | A custom embedded terminal handler | Pretty-printing, automatic reporting, typed behavior | Yes |