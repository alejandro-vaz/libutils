//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;

//> HEAD -> CORE
use core::{
    mem::{
        MaybeUninit,
        forget
    },
    slice::{
        Iter,
        IterMut
    }
};


//^
//^ ITERATORS
//^

//> ITERATORS -> ITERABLE
pub struct Iterable<Type, const N: usize> {
    index: usize,
    length: usize,
    data: MaybeUninit<[Type; N]>
} impl<Type, const N: usize> Drop for Iterable<Type, N> {
    fn drop(&mut self) {while let Some(value) = self.next() {drop(value)}}
}

//> ITERATORS -> ITERATIONS
const impl<Type, const N: usize> Iterator for Iterable<Type, N> {
    type Item = Type;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        return if self.index >= self.length {None} else {
            let value = unsafe {(self.data.as_ptr() as *const Type).add(self.index).read()};
            self.index += 1;
            Some(value)
        }
    }
}

//> ITERATORS -> FROM
impl<Type, const N: usize> FromIterator<Type> for Array<Type, N> {
    fn from_iter<T: IntoIterator<Item = Type>>(iter: T) -> Self {
        let mut array = Self::new();
        array.extend(iter);
        return array;
    }
}

//> ITERATORS -> INTO ITERATOR
const impl<Type, const N: usize> IntoIterator for Array<Type, N> {
    type Item = Type;
    type IntoIter = Iterable<Type, N>;
    fn into_iter(self) -> Self::IntoIter {
        let iterator = Self::IntoIter {
            index: 0,
            length: self.length,
            data: unsafe {MaybeUninit::new(self.data.as_ptr().read())}
        };
        forget(self);
        return iterator;
    }
}

//> ITERATORS -> BORROWED
const impl<'valid, Type, const N: usize> IntoIterator for &'valid Array<Type, N> {
    type Item = &'valid Type;
    type IntoIter = Iter<'valid, Type>;
    fn into_iter(self) -> Self::IntoIter {self.iter()}
}

//> ITERATORS -> BORROWED MUTABLY
const impl<'valid, Type, const N: usize> IntoIterator for &'valid mut Array<Type, N> {
    type Item = &'valid mut Type;
    type IntoIter = IterMut<'valid, Type>;
    fn into_iter(self) -> Self::IntoIter {self.iter_mut()}
}