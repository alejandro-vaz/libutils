# issue

This library exports a type called `Issue` which represents a tiny error wrapper with additional metadata.

This error type is meant to be passed and handled by other libraries, specifically between `libutils-report` and `libutils-terminal`.

Crates should by all means either implement their own type with `Into<Issue>` or use `Issue` directly.

The issue type reflects errors that are meant to be shown to the user, not programming mistakes.

See [the documentation](https://docs.rs/libutils-issue) for more information.