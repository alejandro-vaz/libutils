//^
//^ HEAD
//^

//> HEAD -> MODULES
mod comparisons;
mod conversions;
mod iterators;
mod references;

//> HEAD -> CORE
use core::{
    mem::MaybeUninit,
    ops::Drop,
    fmt::{
        Debug,
        Formatter,
        Result as Format
    },
    hash::{
        Hash,
        Hasher
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
    const fn ptr(&self) -> *const Type {return self.data.as_ptr() as *const Type}
    #[inline]
    const fn mutptr(&mut self) -> *mut Type {return self.data.as_mut_ptr() as *mut Type}
}

//> ARRAY -> IMPLEMENTATION
impl<Type, const N: usize> Array<Type, N> {
    #[inline]
    pub const fn new() -> Self {return Self {
        data: MaybeUninit::uninit(),
        length: 0
    }}
    #[inline]
    pub const unsafe fn arranged(length: usize, data: MaybeUninit<[Type; N]>) -> Self {return Self {
        length: length,
        data: data
    }}
    #[inline]
    pub const fn len(&self) -> usize {return self.length}
    #[inline]
    pub const fn push(&mut self, value: Type) -> () {
        if self.length == N {panic!("array capacity exceeded")}
        unsafe {self.mutptr().add(self.length).write(value)};
        self.length += 1;
    }
    #[inline]
    pub const fn pop(&mut self) -> Option<Type> {return if self.length == 0 {None} else {
        self.length -= 1;
        Some(unsafe {self.mutptr().add(self.length).read()})
    }}
    #[inline]
    pub fn clear(&mut self) -> () {
        for index in 0..self.length {unsafe {self.mutptr().add(index).drop_in_place()}}
        self.length = 0;
    }
    #[inline]
    pub const fn get<'valid>(&'valid self, index: usize) -> Option<&'valid Type> {
        return if self.length <= index {None} else {unsafe {self.ptr().add(index).as_ref()}}
    }
    #[inline]
    pub const fn get_mut<'valid>(&'valid mut self, index: usize) -> Option<&'valid mut Type> {
        return if self.length <= index {None} else {unsafe {self.mutptr().add(index).as_mut()}}
    }
}

//> ARRAY -> DROP
impl<Type, const N: usize> Drop for Array<Type, N> {
    fn drop(&mut self) {self.clear()}
}

//> ARRAY -> DEBUG
impl<Type: Debug, const N: usize> Debug for Array<Type, N> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {Debug::fmt(self.as_ref(), formatter)}
}

//> ARRAY -> HASH
impl<Type: Hash, const N: usize> Hash for Array<Type, N> {
    fn hash<Hashing: Hasher>(&self, state: &mut Hashing) {Hash::hash(self.as_ref(), state)}
}

//> ARRAY -> EXTEND
impl<Type, const N: usize> Extend<Type> for Array<Type, N> {
    fn extend<T: IntoIterator<Item = Type>>(&mut self, iter: T) {
        let mut iterator = iter.into_iter();
        while let Some(item) = iterator.next() {self.push(item)}
    }
}

//> ARRAY -> CLONE
impl<Type: Clone, const N: usize> Clone for Array<Type, N> {
    fn clone(&self) -> Self {return Self::from_iter(self.as_ref().into_iter().cloned())}
}