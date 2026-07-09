# Console

`Console` is a wrapper for I/O operations that are frequently disorganized and all over the place in standard libraries.

Types implementing it are commonly ZSTs that leverage opaque types and mutable statics behind the scenes to offer a good namespaced development experience.

```rust
impl Console for Terminal {
    // ..
}

Terminal.print("hello").sync();
```

## Usage

The main trait to be implemented is the trait `Console`. This trait will offer chaining in some of its operations to other types, like the `impl Update` that's generic and that allows us to either synchronize with the output or ignore now the action we've made.

Other traits also include `Descriptor` and `Metadata` for file handling.

Another special method on `Console` is the `.arguments() -> &[Argument]` method which returns an slice to the arguments provided via CLI. The `Argument` type is a custom type which parses arguments according to their structure so downstream handling is easier.

## When to use it

Very few people should consider implementing their own `Console` type from default. Instead, use `libutils::terminal::Terminal`, which offers a better interface to the standard library.

However, to be able to use `Terminal`, you might need to import the traits from this module.