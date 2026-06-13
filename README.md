# libutils -- general utility library

This library is meant to have types that may be useful for coding executables and libraries that all need shared functionality.

Some of the features of the library may work without the standard library.

| Module | Description | Use cases | Requires `std` |
|:-|:-|:-|:-|
| `cage` | A thread-safe static mutable wrapper | Allocator APIs, safe mutable statics, atomic types | Yes |
| `report` | Comprehensive error wrappers for detailed logs | Compilers, error chains, logs | Yes |