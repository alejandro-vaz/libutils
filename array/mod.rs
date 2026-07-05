//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> LINTS
#![allow(incomplete_features)]

//> HEAD -> FEATURES
#![feature(const_cmp)]
#![feature(const_destruct)]
#![feature(deref_pure_trait)]
#![feature(const_array)]
#![feature(transmute_neo)]
#![feature(const_trait_impl)]
#![feature(box_vec_non_null)]
#![feature(trusted_len)]
#![feature(const_clone)]
#![feature(const_slice_make_iter)]
#![feature(test)]
#![feature(generic_const_exprs)]
#![feature(const_iter)]
#![feature(const_convert)]
#![feature(const_default)]

//> HEAD -> CRATES
extern crate alloc;
extern crate test;

//> HEAD -> MODULES
#[cfg(test)]
mod benches;
mod comparisons;
mod conversions;
mod iterators;
mod references;
#[cfg(test)]
mod tests;

//> HEAD -> CORE
use core::{
    fmt::{
        Debug,
        Formatter,
        Result as Format
    }, 
    mem::{
        MaybeUninit,
        forget
    }, 
    ops::Drop, 
    ptr::copy,
    marker::Destruct
};


//^
//^ ARRAY
//^

//> ARRAY -> STRUCT
pub struct Array<Type, const N: usize> {
    length: usize,
    data: MaybeUninit<[Type; N]>
}

//> ARRAY -> IMPLEMENTATION
impl<Type, const N: usize> Array<Type, N> {
    pub const fn new() -> Self {return Self::default()}
    pub const fn as_ptr(&self) -> *const Type {return self.data.as_ptr().cast()}
    pub const fn as_mut_ptr(&mut self) -> *mut Type {return self.data.as_mut_ptr().cast()}
    pub const fn push(&mut self, value: Type) -> () {
        assert!(self.length != N, "array capacity exceeded");
        unsafe {self.as_mut_ptr().add(self.length).write(value)};
        self.length += 1;
    }
    pub const fn push_mut<'valid>(&'valid mut self, value: Type) -> &'valid mut Type {
        let index = self.length;
        self.push(value);
        return &mut self[index];
    }
    pub const fn pop(&mut self) -> Option<Type> {return if self.length == 0 {None} else {
        self.length -= 1;
        return Some(unsafe {self.as_ptr().add(self.length).read()});
    }}
    pub const fn clear(&mut self) -> () where Type: [const] Destruct {
        let inner = Array::<Type, N> {
            length: self.length,
            data: MaybeUninit::new(unsafe {self.data.as_ptr().read()})
        };
        drop(inner.into_iter());
        self.length = 0;
    }
    pub const fn truncate<const LENGTH: usize>(&mut self) -> () where  Type: [const] Destruct, [(); N - LENGTH]: {
        Array::<Type, {N - LENGTH}> {
            length: self.length - LENGTH,
            data: MaybeUninit::new(unsafe {self.data.as_ptr().add(LENGTH).cast::<[Type; N - LENGTH]>().read()})
        }.clear();
        self.length = LENGTH;
    }
    pub const fn insert(&mut self, index: usize, value: Type) -> () {
        assert!(index <= self.length, "tried to insert out of bounds");
        assert!(self.length != N, "array capacity exceeded");
        let pointer = unsafe {self.as_mut_ptr().add(index)};
        unsafe {copy(pointer, pointer.add(1), self.length - index)}
        unsafe {pointer.write(value)}
        self.length += 1;
    }
    pub const fn insert_mut<'valid>(&'valid mut self, index: usize, value: Type) -> &'valid mut Type {
        self.insert(index, value);
        return &mut self[index];
    }
    pub const fn remove(&mut self, index: usize) -> Type {
        assert!(index < self.length, "tried to remove out of bounds");
        let pointer = unsafe {self.as_mut_ptr().add(index)};
        let value = unsafe {pointer.read()};
        unsafe {copy(pointer.add(1), pointer, self.length - 1 - index)};
        self.length -= 1;
        return value;
    }
    pub const fn retain(
        &mut self, 
        mut closure: impl [const] FnMut(&mut Type) -> bool + [const] Destruct
    ) -> () where Type: [const] Destruct {
        let mut offset = 0;
        let mut position = 0;
        while position < self.length {
            let mut item = unsafe {self.as_mut_ptr().add(position).read()};
            if closure(&mut item) {
                if offset == 0 {
                    forget(item)
                } else {
                    unsafe {self.as_mut_ptr().add(position).sub(offset).write(item)};
                }
            } else {
                drop(item);
                offset += 1;
            }
            position += 1;
        }
    }
}

//> ARRAY -> DROP
const impl<Type: [const] Destruct, const N: usize> Drop for Array<Type, N> {
    fn drop(&mut self) {self.clear()}
}

//> ARRAY -> DEBUG
impl<Type: Debug, const N: usize> Debug for Array<Type, N> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {Debug::fmt(self.as_ref(), formatter)}
}

//> ARRAY -> EXTEND
impl<Type, const N: usize> Extend<Type> for Array<Type, N> {
    fn extend<T: IntoIterator<Item = Type>>(&mut self, iter: T) {for item in iter {self.push(item)}}
}

//> ARRAY -> CLONE
const impl<Type: [const] Clone, const N: usize> Clone for Array<Type, N> {
    fn clone(&self) -> Self {
        let mut array = Array::new();
        let mut position = 0;
        while position < self.length {
            let item = &self[position];
            array.push(item.clone());
            position += 1;
        }
        return array;
    }
}

//> ARRAY -> DEFAULT
const impl<Type, const N: usize> Default for Array<Type, N> {
    fn default() -> Self {return Self {
        data: MaybeUninit::uninit(),
        length: 0
    }}
}