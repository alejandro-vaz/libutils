# libutils -- general utility library

This library is meant to have types that may be useful for coding executables and libraries that all need shared functionality.

| Module | Description | Use cases | Requires `std` |
|:-|:-|:-|:-|
| `array` | A maximum-sized array stored on the stack | Upper-bounded length arrays, unknown amount of elements, speed | No |
| `cage` | A thread-safe static mutable wrapper | Allocator APIs, safe mutable statics, atomic types | Yes |
| `diff` | A bytearray diff | Incremental systems, text replacement, change tracking | No |
| `issue` | A easy-to-use customizable error type | Error handling, failures, issue reporting | No |
| `problem` | A problem wrapper with additional metadata | Error handling, issue information, logs | Yes |
| `report` | Comprehensive error wrappers for detailed logs | Compilers, error chains, logs | Yes |
| `terminal` | A custom embedded terminal handler | Pretty-printing, automatic reporting, typed behavior | Yes |