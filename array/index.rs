//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;

//> HEAD -> CORE
use core::ops::{
    Index,
    IndexMut,
    Range,
    RangeFrom,
    RangeFull,
    RangeInclusive,
    RangeTo,
    RangeToInclusive
};


//^
//^ USIZE
//^

//> USIZE -> IMPLEMENTATION
const impl<Type, const N: usize> Index<usize> for Array<Type, N> {
    type Output = Type;
    fn index(&self, index: usize) -> &Self::Output {self.get(index).unwrap()}
}

//> USIZE -> MUT IMPLEMENTATION
const impl<Type, const N: usize> IndexMut<usize> for Array<Type, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {self.get_mut(index).unwrap()}
}


//^
//^ RANGE
//^

//> RANGE -> IMPLEMENTATION
const impl<Type, const N: usize> Index<Range<usize>> for Array<Type, N> {
    type Output = [Type];
    fn index(&self, index: Range<usize>) -> &Self::Output {self.as_ref().index(index)}
}

//> RANGE -> MUT IMPLEMENTATION
const impl<Type, const N: usize> IndexMut<Range<usize>> for Array<Type, N> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {self.as_mut().index_mut(index)}
}


//^
//^ RANGEFROM
//^

//> RANGEFROM -> IMPLEMENTATION
const impl<Type, const N: usize> Index<RangeFrom<usize>> for Array<Type, N> {
    type Output = [Type];
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {self.as_ref().index(index)}
}

//> RANGEFROM -> MUT IMPLEMENTATION
const impl<Type, const N: usize> IndexMut<RangeFrom<usize>> for Array<Type, N> {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {self.as_mut().index_mut(index)}
}


//^
//^ RANGEFULL
//^

//> RANGEFULL -> IMPLEMENTATION
const impl<Type, const N: usize> Index<RangeFull> for Array<Type, N> {
    type Output = [Type];
    fn index(&self, index: RangeFull) -> &Self::Output {self.as_ref().index(index)}
}

//> RANGEFULL -> MUT IMPLEMENTATION
const impl<Type, const N: usize> IndexMut<RangeFull> for Array<Type, N> {
    fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {self.as_mut().index_mut(index)}
}


//^
//^ RANGEINCLUSIVE
//^

//> RANGEINCLUSIVE -> IMPLEMENTATION
const impl<Type, const N: usize> Index<RangeInclusive<usize>> for Array<Type, N> {
    type Output = [Type];
    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {self.as_ref().index(index)}
}

//> RANGEINCLUSIVE -> MUT IMPLEMENTATION
const impl<Type, const N: usize> IndexMut<RangeInclusive<usize>> for Array<Type, N> {
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {self.as_mut().index_mut(index)}
}


//^
//^ RANGETO
//^

//> RANGETO -> IMPLEMENTATION
const impl<Type, const N: usize> Index<RangeTo<usize>> for Array<Type, N> {
    type Output = [Type];
    fn index(&self, index: RangeTo<usize>) -> &Self::Output {self.as_ref().index(index)}
}

//> RANGETO -> MUT IMPLEMENTATION
const impl<Type, const N: usize> IndexMut<RangeTo<usize>> for Array<Type, N> {
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {self.as_mut().index_mut(index)}
}


//^
//^ RANGETOINCLUSIVE
//^

//> RANGETOINCLUSIVE -> IMPLEMENTATION
const impl<Type, const N: usize> Index<RangeToInclusive<usize>> for Array<Type, N> {
    type Output = [Type];
    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {self.as_ref().index(index)}
}

//> RANGETOINCLUSIVE -> MUT IMPLEMENTATION
const impl<Type, const N: usize> IndexMut<RangeToInclusive<usize>> for Array<Type, N> {
    fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut Self::Output {self.as_mut().index_mut(index)}
}