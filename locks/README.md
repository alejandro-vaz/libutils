# cagelock

`Cage<Type>` is a reader-writer lock wrapper with a simple and safe API.

## Usage

```rust
#![feature(const_default)]
#![feature(const_trait_impl)]

use locks::Mutex;

static MUTABLE: Mutex<u8> = Mutex::default();

MUTABLE.with(|variable| *variable = 1); // get a mutable reference

let x = MUTABLE.get(); // get with a guard
```

The `Cage` type offers a wrapper around `RwLock` from the standard library with a simplified API.

## Performance

This type's performance largely dependent on the `RwLock` implementation.

## When to use it

The type is useful in two situations:
- Static mutable variables: synchronizes access
- Interior mutability: enforces borrowing rules at runtime
