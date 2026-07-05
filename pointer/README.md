# pointer

This crate contains the type `Pointer` which is a smart wrapper around `Option<NonNull<Type>>` which offers a safer and more reliable API.

Most of the API for the pointer is written in const-code which means that it is able to be evaluated by the compiler.

This type also implements the traits `Send` and `Sync`, making it suitable for multithreaded contexts.

See [the documentation](https://docs.rs/libutils-pointer) for more information.