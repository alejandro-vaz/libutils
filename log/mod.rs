//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> FEATURES
#![feature(allocator_api)]
#![feature(const_trait_impl)]
#![feature(const_default)]
#![feature(const_convert)]
#![feature(const_slice_make_iter)]
#![feature(const_iter)]
#![feature(const_result_unwrap_unchecked)]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
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
    alloc::{
        Allocator,
        Layout
    },
    slice::Iter
};

//> HEAD -> ALLOC
use alloc::alloc::Global;

//> HEAD -> POINTER
use pointer::Pointer;


//^
//^ LOG
//^

//> LOG -> STRUCT
#[derive(Clone)]
pub struct Log<Type> {
    pointer: Pointer<Type>,
    length: usize,
    capacity: usize
}

//> LOG -> IMPLEMENTATION
impl<Type> Log<Type> {
    #[inline]
    fn grow(&mut self) -> () {
        self.pointer = match self.pointer.take() {
            None => Global.allocate(Layout::from_size_align(size_of::<Type>(), align_of::<Type>()).unwrap()),
            Some(pointer) => unsafe {Global.grow(
                pointer.cast(), 
                Layout::from_size_align(self.capacity * size_of::<Type>(), align_of::<Type>()).unwrap(), 
                Layout::from_size_align(self.capacity * 2 * size_of::<Type>(), align_of::<Type>()).unwrap()
            )}
        }.unwrap().cast().into();
        self.capacity = (self.capacity * 2).max(1);
    }
    pub const fn new() -> Self {return Self::default()}
    #[inline]
    pub const fn len(&self) -> usize {return self.length}
    pub const fn is_empty(&self) -> bool {return self.length == 0}
    #[inline]
    pub fn push(&mut self, value: Type) -> () {
        if self.length == self.capacity || self.pointer.is_null() {self.grow()}
        unsafe {self.pointer.add(self.length).write(value)};
        self.length += 1;
    }
    #[inline]
    pub const fn iter<'valid>(&'valid self) -> Iter<'valid, Type> {return self.as_ref().iter()}
}

//> LOG -> DEFAULT
const impl<Type> Default for Log<Type> {
    fn default() -> Self {return Self {
        pointer: Pointer::default(),
        length: 0,
        capacity: 0
    }}
}

//> LOG -> EXTEND
impl<Type> Extend<Type> for Log<Type> {
    #[inline]
    fn extend<T: IntoIterator<Item = Type>>(&mut self, iter: T) {iter.into_iter().for_each(|item| self.push(item))}
}

//> LOG -> DEBUG
impl<Type: Debug> Debug for Log<Type> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {Debug::fmt(self.as_ref(), formatter)}
}

//> LOG -> DROP
impl<Type> Drop for Log<Type> {
    fn drop(&mut self) {if let Some(pointer) = self.pointer.take() {
        for index in 0..self.length {drop(unsafe {pointer.add(index).read()})}
        unsafe {Global.deallocate(pointer.cast(), Layout::from_size_align(self.capacity * size_of::<Type>(), align_of::<Type>()).unwrap());}
    }}
}