# Issue

`Issue` is a small but detailed error wrapper that can be handled by upstream code and can be built from other types.

## The problem it solves

Easy, fast reporting to the user via a CLI with enough information.

## Usage

This type is not meant to be used by ordinary people writing code, but by library backends defining custom errors:

```rust
use libutils_issue::Issue;

enum MyError {
    FirstCause,
    SecondCause
}

impl Into<Issue> for MyError {
    fn into(self) -> Issue {
        return match self {
            MyError::FirstCause => Issue {name: "error 1", ..},
            MyError::SecondCause => Issue {name: "error 2", ..}
        }
    }
}
```

Then, libraries might use it in two ways:
- By returning it to the user on `Result<Type, Issue>` with standard try-trait workflow,
- By submitting it to a type that implements `Console` (see `libutils-console` for that)

## When to use it

This error type is meant to be used for errors whose failure might interest the user. Common examples are:
- A file doesn't exist
- The input wasn't correct
- Couldn't fetch

Examples for instances in which you should not use it:
- The program internals failed

For that second case, you should likely use `.unwrap()` or `.expect(&'static str)` instead.