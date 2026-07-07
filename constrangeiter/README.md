# Constrangeiter

This small utility crate provides const-evaluable iterators for ranges using the new range API.

## Problem it solves

Current ranges in rust, either with the old API (`core::ops::Range`) or the new API (`core::range::Range`) do not implement `const IntoIterator` nor the produced iterators implement `const Iterator`. That makes them impossible to evaluate in const contexts.

For that reason, we made a `ConstIntoIterator` trait implemented for the ranges (normal, inclusive, and only starting), that is naturally const and whose produced iterators are const-evaluable as well.

## Usage

```rust
use libutils::constrangeiter::ConstIntoIterator;

const {
    for index in (0..6).const_into_iter() {
        // ..
    }
}
```

The method `.const_into_iter() -> impl const Iterator` must be explicitly called (it is not called by the rust machinery) and it converts the range into a constant iterator.

## When to use it

This is useful in contexts like constant iteration, and it is used in parts of `libutils_array`.

For example, if we want to drop some elements in an array-like storage, we first want to iterate over them and drop each one by one. That process would be const iff `Type: [const] Destruct`, but if range iteration is never const, we don't get to ever make it const even if the type has a constant drop implementation.

> This crate requires the `#![feature(new_range)]` attribute, as `const ConstIntoIterator` has not been implemented for the old range types.