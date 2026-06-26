//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Log;

//> HEAD -> CORE
use core::slice::Iter;

//> HEAD -> STD
use std::vec::IntoIter;


//^
//^ ITERATORS
//^

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
    type IntoIter = IntoIter<Type>;
    fn into_iter(self) -> Self::IntoIter {return self.data.into_iter()}
}

//> ITERATORS -> BORROWED
impl<'valid, Type> const IntoIterator for &'valid Log<Type> {
    type Item = &'valid Type;
    type IntoIter = Iter<'valid, Type>;
    fn into_iter(self) -> Self::IntoIter {self.iter()}
}

//> ITERATORS -> METHODS
impl<Type> Log<Type> {
    #[inline]
    pub const fn iter<'valid>(&'valid self) -> Iter<'valid, Type> {return self.data.iter()}
}