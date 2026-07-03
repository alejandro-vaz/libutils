# libutils -- a general utility library

This library is meant to have types that may be useful for coding executables and libraries that all need shared functionality.

| Module | Description | Use cases | Requires `std` |
|:-|:-|:-|:-|
| `array` | A maximum-sized array stored on the stack | Upper-bounded length arrays, speed | No |
| `cage` | A thread-safe static mutable wrapper | Safe mutable statics, atomic types | Yes |
| `diff` | A bytearray diff | Incremental systems, text replacement | No |
| `issue` | A easy-to-use customizable error type | Error handling, issue reporting | No |
| `log` | A write-only vec-like buffer | Logs, no removals | No |
| `pointer` | A typed custom pointer to data | Pointers, typed behavior | No |
| `report` | A custom error wrapper for detailed logs | Compilers, error chains | Yes |
| `terminal` | A custom embedded terminal handler | Pretty-printing, typed behavior | Yes |
| `threat` | An error wrapper with additional metadata | Error handling, logs | No |