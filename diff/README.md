# diff

This crate contains the type `Diff` which takes two byte arrays and stores the process needed to convert the output of a terminal from the starting array to the new changed array.

It works by finding an split point in which the sequences differ, erasing up to that point, and rewriting with the new data.

This type is useful for dynamic terminal output.

See [the documentation](https://docs.rs/libutils-diff) for more information.