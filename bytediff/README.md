# bytediff

Bytediff is a small utility type that represents the information required to update a bytearray to some new state in a terminal context.

## What problem it solves

This type allows for updating terminal output without overwriting the full context by leveraging symmetries and differences in outputs.

## Usage

```rust
use libutils::bytediff::Diff;

const BEFORE: [u8; 5] = [0, 1, 2, 3, 4];
const AFTER: [u8; 3] = [0, 1, 2];

let diff = Diff::new(&BEFORE, &AFTER);
```

`Diff` implements `Into<Vec<u8>>` which then can be written to some output stream. It will make sure to remove bytes not present anymore, and write the new ones (hence its lifetime is covariant over that of the second array);

## When to use it

This type is specially useful when you're building CLIs that require some sort of active state updating. For example, we might have a progressbar:

```
[          ]
```

that shows ups like that, but then eventually it fills up:

```
[#####     ]
```

```
[##########]
```

Every time we need to update the bar, we need to move the terminal cursor behind and write some bytes. That is automatically handled by `Diff` when we convert it to a `Vec<u8>`, so it only makes the necessary writes to update our destination.