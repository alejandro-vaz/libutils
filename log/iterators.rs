//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Log;

//> HEAD -> CORE
use core::{
    slice::Iter,
    mem::ManuallyDrop,
    alloc::{
        Layout,
        Allocator
    }
};

//> HEAD -> CRATE
use crate::pointer::Pointer;

//> HEAD -> STD
use std::alloc::Global;


//^
//^ ITERATORS
//^

//> ITERABLE
pub struct Iterable<Type> {
    pointer: Pointer<'static, Type>,
    length: usize,
    index: usize,
    layout: Layout
} impl<Type> Iterator for Iterable<Type> {
    type Item = Type;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = if self.index < self.length {
            Some(unsafe {self.pointer.add(self.index).read()?})
        } else {None};
        self.index += 1;
        return value;
    }
} impl<Type> Drop for Iterable<Type> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
        if let Some(pointer) = self.pointer.take() {
            unsafe {Global.deallocate(pointer.cast(), self.layout)}
        }
    }
}

//> ITERATORS -> FROM
impl<Type> FromIterator<Type> for Log<Type> {
    fn from_iter<T: IntoIterator<Item = Type>>(iter: T) -> Self {
        let mut log = Log::new();
        log.extend(iter);
        return log;
    }
}

//> ITERATORS -> INTO ITERATOR
impl<Type> IntoIterator for Log<Type> {
    type Item = Type;
    type IntoIter = Iterable<Type>;
    fn into_iter(self) -> Self::IntoIter {
        let instance = ManuallyDrop::new(self);
        return Iterable {
            pointer: instance.pointer,
            length: instance.length,
            index: 0,
            layout: Layout::from_size_align(instance.capacity * size_of::<Type>(), align_of::<Type>()).unwrap()
        }
    }
}

//> ITERATORS -> BORROWED
const impl<'valid, Type> IntoIterator for &'valid Log<Type> {
    type Item = &'valid Type;
    type IntoIter = Iter<'valid, Type>;
    fn into_iter(self) -> Self::IntoIter {self.iter()}
}

//> ITERATORS -> METHODS
impl<Type> Log<Type> {
    #[inline]
    pub const fn iter<'valid>(&'valid self) -> Iter<'valid, Type> {return self.as_ref().iter()}
}