//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    ConstIntoIterator,
    Target
};

//> HEAD -> CORE
use core::{
    iter::{
        ExactSizeIterator,
        FusedIterator,
        TrustedLen
    },
    range::RangeFrom,
    marker::Destruct
};


//^
//^ ITERATOR
//^

//> ITERATOR -> ITERABLE
#[derive(Debug)]
pub struct Iterable<Type: const Target> where usize: const TryFrom<Type>, <usize as TryFrom<Type>>::Error: const Destruct {
    start: Option<Type>
}

//> ITERATOR -> IMPLEMENTATION
const impl<Type: const Target> Iterator for Iterable<Type> where usize: const TryFrom<Type>, <usize as TryFrom<Type>>::Error: const Destruct {
    type Item = Type;
    fn next(&mut self) -> Option<Self::Item> {
        let now = self.start?;
        if now == Type::MAX {self.start = None} else {self.start.as_mut()?.add_assign(Type::ONE)}
        return Some(now);
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        return Type::MAX.hint(self.start.unwrap_or(Type::MAX));
    }
}

//> ITERATOR -> EXACT SIZE
impl<Type: const Target> ExactSizeIterator for Iterable<Type> where usize: const TryFrom<Type>, <usize as TryFrom<Type>>::Error: const Destruct {}

//> ITERATOR -> FUSED
impl<Type: const Target> FusedIterator for Iterable<Type> where usize: const TryFrom<Type>, <usize as TryFrom<Type>>::Error: const Destruct {}

//> ITERATOR -> TRUSTED LEN
unsafe impl<Type: const Target> TrustedLen for Iterable<Type> where usize: const TryFrom<Type>, <usize as TryFrom<Type>>::Error: const Destruct {}


//^
//^ RANGE
//^

//> RANGE -> IMPLEMENTATION
const impl<Type: const Target> ConstIntoIterator for RangeFrom<Type> where usize: const TryFrom<Type>, <usize as TryFrom<Type>>::Error: const Destruct {
    type Item = Type;
    type IntoIter = Iterable<Type>;
    fn const_into_iter(self) -> Self::IntoIter {return Iterable {
        start: Some(self.start)
    }}
}