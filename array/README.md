# array

`Array<Type, N>` is an stack-allocated vector of maximum capacity `N`.

It exposes an API similar to `Vec` and allows for the same operations, but panics if its length exceeds capacity.

Since most of its implementations are const, it is very fast.

It is well-suited for contexts in which you know the capacity of a vector isn't going to exceed a certain small number.

See [the documentation](https://docs.rs/libutils-array) for more information.