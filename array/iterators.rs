//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;

//> HEAD -> CORE
use core::mem::{
    MaybeUninit,
    forget
};


//^
//^ ITERATORS
//^

//> ITERATORS -> ITERABLE
pub struct Iterable<Type, const N: usize> {
    index: usize,
    length: usize,
    data: MaybeUninit<[Type; N]>
}

//> ITERATORS -> ITERATIONS
impl<Type, const N: usize> Iterator for Iterable<Type, N> {
    type Item = Type;
    fn next(&mut self) -> Option<Self::Item> {
        return if self.index >= self.length {None} else {
            let value = unsafe {(self.data.as_ptr() as *const Type).add(self.index).read()};
            self.index += 1;
            return Some(value);
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
impl<Type, const N: usize> IntoIterator for Array<Type, N> {
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