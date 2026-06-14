# libutils -- general utility library

This library is meant to have types that may be useful for coding executables and libraries that all need shared functionality.

| Module | Description | Use cases | Requires `std` |
|:-|:-|:-|:-|
| `cage` | A thread-safe static mutable wrapper | Allocator APIs, safe mutable statics, atomic types | Yes |
| `report` | Comprehensive error wrappers for detailed logs | Compilers, error chains, logs | Yes |
| `array` | A maximum-sized array stored on the stack | Upper-bounded length arrays, unknown amount of elements, speed | No |