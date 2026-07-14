# cagelock

`Cage<Type>` is a reader-writer lock wrapper with a simple and safe API.

## Usage

```rust
use cagelock::Cage;

static MUTABLE: Cage<u8> = Cage::default();

MUTABLE.write(|variable| *variable = 1); // get a mutable reference

let x = MUTABLE.get(); // copy
```

The `Cage` type offers a wrapper around `RwLock` from the standard library with a simplified API.

## Performance

This type's performance largely dependent on the `RwLock` implementation.

## When to use it

The type is useful in two situations:
- Static mutable variables: synchronizes access
- Interior mutability: enforces borrowing rules at runtime