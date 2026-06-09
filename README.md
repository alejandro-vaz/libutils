# libutils -- general utility library

This library is meant to have types that may be useful for coding executables and libraries that all need shared functionality.

Some of the features of the library may work without `std`.

| Type | Description | Use cases | Requires `std` |
|:-|:-|:-|:-|
| `Cage` | A thread-safe static mutable wrapper | Allocator APIs, safe mutable statics, atomic types | Yes |