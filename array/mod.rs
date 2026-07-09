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
#![feature(const_closures)]
#![feature(const_trait_impl)]
#![feature(box_vec_non_null)]
#![feature(trusted_len)]
#![feature(const_clone)]
#![feature(new_range)]
#![feature(const_slice_make_iter)]
#![feature(test)]
#![feature(const_ops)]
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
    marker::Destruct, 
    mem::{
        MaybeUninit,
        forget
    }, 
    ops::{
        Drop, 
        SubAssign
    }, 
    ptr::copy
};

//> HEAD -> CONSTRANGEITER
use libutils_constrangeiter::ConstIntoIterator;


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
    pub const fn is_full(&self) -> bool {return self.length == N}
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
    pub const fn clear(&mut self) -> () where Type: [const] Destruct {self.truncate(0)}
    pub const fn truncate(&mut self, length: usize) -> () where Type: [const] Destruct {
        for index in (length..self.length).const_into_iter() {
            drop(unsafe {self.as_ptr().add(index).read()})
        }
        self.length = length;
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
    pub const fn swap_remove(&mut self, index: usize) -> Type {
        assert!(index < self.length, "tried to remove out of bounds");
        let pointer = unsafe {self.as_mut_ptr().add(index)};
        let value = unsafe {pointer.read()};
        unsafe {copy(self.as_ptr().add(self.length).sub(1), pointer, 1)}
        self.length -= 1;
        return value;
    }
    pub const fn retain(
        &mut self, 
        mut closure: impl [const] FnMut(&mut Type) -> bool + [const] Destruct
    ) -> () where Type: [const] Destruct {
        let mut offset = 0;
        for position in (0..self.length).const_into_iter() {
            let mut item = unsafe {self.as_mut_ptr().add(position).read()};
            if closure(&mut item) {
                if offset == 0 {
                    forget(item);
                } else {
                    unsafe {self.as_mut_ptr().add(position).sub(offset).write(item)}
                }
            } else {
                drop(item);
                offset += 1;
            }
        }
        self.length.sub_assign(offset);
    }
    pub const fn dedup(&mut self) -> () where Type: [const] PartialEq<Type> + [const] Destruct {
        self.dedup_by(const |first, second| (first as &Type).eq(second as &Type));
    }
    pub const fn dedup_by(
        &mut self, 
        mut decider: impl [const] FnMut(&mut Type, &mut Type) -> bool + [const] Destruct
    ) -> () where Type: [const] Destruct {
        if self.length < 2 {return}
        let mut offset = 0;
        let mut first = None;
        for position in (0..=self.length - 1).const_into_iter() {
            let mut second = unsafe {self.as_ptr().add(position).read()};
            if first.is_none() {
                first = Some(second);
                continue;
            }
            if decider(first.as_mut().unwrap(), &mut second) {
                drop(second);
                offset += 1;
            } else {
                if offset == 0 {
                    forget(first.replace(second).unwrap());
                } else {
                    unsafe {self.as_mut_ptr().add(position).sub(offset).write(second)};
                    forget(first.replace(unsafe {self.as_mut_ptr().add(position).sub(offset).read()}).unwrap());
                }
            }
        }
        self.length.sub_assign(offset);
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
        for position in (0..self.length).const_into_iter() {array.push(self[position].clone())}
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