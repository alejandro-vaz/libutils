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
        DoubleEndedIterator,
        ExactSizeIterator,
        FusedIterator,
        TrustedLen
    },
    range::Range,
    marker::Destruct
};


//^
//^ ITERATOR
//^

//> ITERATOR -> ITERABLE
#[derive(Debug)]
pub struct Iterable<Type: const Target> where usize: const TryFrom<Type>, <usize as TryFrom<Type>>::Error: const Destruct {
    start: Type,
    end: Type
}

//> ITERATOR -> IMPLEMENTATION
const impl<Type: const Target> Iterator for Iterable<Type> where usize: const TryFrom<Type>, <usize as TryFrom<Type>>::Error: const Destruct {
    type Item = Type;
    fn next(&mut self) -> Option<Self::Item> {return if self.start < self.end {
        let now = self.start;
        self.start.add_assign(Type::ONE);
        Some(now)
    } else {None}}
    fn size_hint(&self) -> (usize, Option<usize>) {
        return if self.start < self.end {self.end.hint(self.start)} else {(0, Some(0))};
    }
}

//> ITERATOR -> DOUBLE ENDED
const impl<Type: const Target> DoubleEndedIterator for Iterable<Type> where usize: const TryFrom<Type>, <usize as TryFrom<Type>>::Error: const Destruct {
    fn next_back(&mut self) -> Option<Self::Item> {return if self.start < self.end {
        self.end.sub_assign(Type::ONE);
        return Some(self.end)
    } else {None}}
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
const impl<Type: const Target> ConstIntoIterator for Range<Type> where usize: const TryFrom<Type>, <usize as TryFrom<Type>>::Error: const Destruct {
    type Item = Type;
    type IntoIter = Iterable<Type>;
    fn const_into_iter(self) -> Self::IntoIter {return Iterable {
        start: self.start,
        end: self.end
    }}
}