# terminal

The static `TERMINAL: Terminal` is an implementation of `Console` from `libutils-console` that uses the standard library.

It aims to replace the API for handling that the stdlib offers by giving a simpler interface programmers can sympathize with faster.

It makes use of thread synchronization via reader-writer lock to maintain interior mutability.

See [the documentation](https://docs.rs/libutils-terminal) for more information.