# libutils -- general utility library

This library is meant to have types that may be useful for coding executables and libraries that all need shared functionality.

Some of the features of the library may work without the standard library.

| Type | Description | Use cases | Requires `std` |
|:-|:-|:-|:-|
| `Cage` | A thread-safe static mutable wrapper | Allocator APIs, safe mutable statics, atomic types | Yes |
| `Report` / `Action` | Comprehensive error wrappers for detailed logs | Compilers, error chains, logs | Yes |