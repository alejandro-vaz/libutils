# cage

`Cage<Type>` is a reader-writer lock with a simple and safe API.

It supports common methods for reading, writing, replacing, and getting the inner value.

This type specifically solves those cases in which you need safe static mutables for global variables in your code.

See [the documentation](https://docs.rs/libutils-cage) for more information.