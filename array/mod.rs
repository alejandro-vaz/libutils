//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> FEATURES
#![feature(const_cmp)]
#![feature(const_trait_impl)]
#![feature(const_slice_make_iter)]
#![feature(const_index)]
#![feature(const_iter)]
#![feature(const_convert)]
#![feature(const_default)]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod comparisons;
mod conversions;
mod index;
mod iterators;
mod references;
#[cfg(test)]
mod tests;

//> HEAD -> CORE
use core::{
    mem::MaybeUninit,
    ptr::{
        copy,
        NonNull
    },
    ops::Drop,
    fmt::{
        Debug,
        Formatter,
        Result as Format
    },
    slice::{
        Iter,
        IterMut
    }
};


//^
//^ ARRAY
//^

//> ARRAY -> STRUCT
pub struct Array<Type, const N: usize> {
    length: usize,
    data: MaybeUninit<[Type; N]>
}

//> ARRAY -> INTERNALS
impl<Type, const N: usize> Array<Type, N> {
    #[inline]
    const fn pointer(&mut self) -> NonNull<Type> {return NonNull::new(self.data.as_mut_ptr()).unwrap().cast()}
}

//> ARRAY -> IMPLEMENTATION
impl<Type, const N: usize> Array<Type, N> {
    #[inline]
    pub const fn capacity(&self) -> usize {return N}
    #[inline]
    pub const fn is_empty(&self) -> bool {return self.length == 0}
    #[inline]
    pub const fn new() -> Self {return Self::default()}
    #[inline]
    pub const fn len(&self) -> usize {return self.length}
    #[inline]
    pub const fn push(&mut self, value: Type) -> () {
        assert!(self.length != N, "array capacity exceeded");
        unsafe {self.pointer().add(self.length).write(value)};
        self.length += 1;
    }
    #[inline]
    pub const fn pop(&mut self) -> Option<Type> {return if self.length == 0 {None} else {
        self.length -= 1;
        Some(unsafe {self.pointer().add(self.length).read()})
    }}
    #[inline]
    pub fn clear(&mut self) -> () {
        for index in 0..self.length {unsafe {drop(self.pointer().add(index).read())}}
        self.length = 0;
    }
    #[inline]
    pub const fn get<'valid>(&'valid self, index: usize) -> Option<&'valid Type> {
        return if self.length <= index {None} else {unsafe {(self.data.as_ptr() as *const Type).add(index).as_ref()}}
    }
    #[inline]
    pub const fn get_mut<'valid>(&'valid mut self, index: usize) -> Option<&'valid mut Type> {
        return if self.length <= index {None} else {Some(unsafe {self.pointer().add(index).as_mut()})}
    }
    #[inline]
    pub const fn insert(&mut self, index: usize, value: Type) -> () {
        assert!(index <= self.length, "tried to insert out of bounds");
        assert!(self.length != N, "array capacity exceeded");
        let pointer = unsafe {self.pointer().add(index)};
        unsafe {copy(pointer.as_ptr(), pointer.add(1).as_ptr(), self.length - index)}
        unsafe {pointer.write(value)}
        self.length += 1;
    }
    #[inline]
    pub const fn remove(&mut self, index: usize) -> Type {
        assert!(index <= self.length, "tried to insert out of bounds");
        let pointer = unsafe {self.pointer().add(index)};
        let value = unsafe {pointer.read()};
        unsafe {copy(pointer.add(1).as_ptr(), pointer.as_ptr(), self.length - 1 - index)};
        self.length -= 1;
        return value;
    }
    #[inline]
    pub const fn iter<'valid>(&'valid self) -> Iter<'valid, Type> {return self.as_ref().iter()}
    #[inline]
    pub const fn iter_mut<'valid>(&'valid mut self) -> IterMut<'valid, Type> {return self.as_mut().iter_mut()}
}

//> ARRAY -> DROP
impl<Type, const N: usize> Drop for Array<Type, N> {
    #[inline]
    fn drop(&mut self) {self.clear()}
}

//> ARRAY -> DEBUG
impl<Type: Debug, const N: usize> Debug for Array<Type, N> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {Debug::fmt(self.as_ref(), formatter)}
}

//> ARRAY -> EXTEND
impl<Type, const N: usize> Extend<Type> for Array<Type, N> {
    #[inline]
    fn extend<T: IntoIterator<Item = Type>>(&mut self, iter: T) {for item in iter {self.push(item)}}
}

//> ARRAY -> CLONE
impl<Type: Clone, const N: usize> Clone for Array<Type, N> {
    #[inline]
    fn clone(&self) -> Self {return Self::from_iter(self.as_ref().into_iter().cloned())}
}

//> ARRAY -> DEFAULT
const impl<Type, const N: usize> Default for Array<Type, N> {
    #[inline]
    fn default() -> Self {return Self {
        data: MaybeUninit::uninit(),
        length: 0
    }}
}