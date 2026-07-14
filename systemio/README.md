# SystemIO

`SystemIO` is a wrapper for I/O operations that are frequently disorganized and all over the place in standard libraries.

Types implementing it are commonly ZSTs that leverage opaque types and mutable statics behind the scenes to offer a good namespaced development experience.

```rust
use systemio::SystemIO;

impl SystemIO for Terminal {
    // ..
}

Terminal::print("hello").sync();
```

## Usage

The main trait to be implemented is the trait `SystemIO`. This trait will offer chaining in some of its operations to other types, like the `impl Update` that's generic and that allows us to either synchronize with the output or ignore now the action we've made.

Other traits also include `Descriptor` and `Metadata` for file handling.

Another special associated function on `SystemIO` is `::arguments() -> &[Argument]` which returns an slice to the arguments provided via CLI. The `Argument` type is a custom type which parses arguments according to their structure so downstream handling is easier.

## When to use it

Very few people should consider implementing their own `SystemIO` type from default. Instead, use `System` from the crate [`systemstd`](https://crates.io/crates/systemstd), which offers a better interface to the standard library.

However, to be able to use `System`, you might need to import the traits from this module.