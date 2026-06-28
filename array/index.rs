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
impl<Type, const N: usize> const Index<usize> for Array<Type, N> {
    type Output = Type;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {self.get(index).unwrap()}
}

//> USIZE -> MUT IMPLEMENTATION
impl<Type, const N: usize> const IndexMut<usize> for Array<Type, N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {self.get_mut(index).unwrap()}
}


//^
//^ RANGE
//^

//> RANGE -> IMPLEMENTATION
impl<Type, const N: usize> const Index<Range<usize>> for Array<Type, N> {
    type Output = [Type];
    #[inline]
    fn index(&self, index: Range<usize>) -> &Self::Output {self.as_ref().index(index)}
}

//> RANGE -> MUT IMPLEMENTATION
impl<Type, const N: usize> const IndexMut<Range<usize>> for Array<Type, N> {
    #[inline]
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {self.as_mut().index_mut(index)}
}


//^
//^ RANGEFROM
//^

//> RANGEFROM -> IMPLEMENTATION
impl<Type, const N: usize> const Index<RangeFrom<usize>> for Array<Type, N> {
    type Output = [Type];
    #[inline]
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {self.as_ref().index(index)}
}

//> RANGEFROM -> MUT IMPLEMENTATION
impl<Type, const N: usize> const IndexMut<RangeFrom<usize>> for Array<Type, N> {
    #[inline]
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {self.as_mut().index_mut(index)}
}


//^
//^ RANGEFULL
//^

//> RANGEFULL -> IMPLEMENTATION
impl<Type, const N: usize> const Index<RangeFull> for Array<Type, N> {
    type Output = [Type];
    #[inline]
    fn index(&self, index: RangeFull) -> &Self::Output {self.as_ref().index(index)}
}

//> RANGEFULL -> MUT IMPLEMENTATION
impl<Type, const N: usize> const IndexMut<RangeFull> for Array<Type, N> {
    #[inline]
    fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {self.as_mut().index_mut(index)}
}


//^
//^ RANGEINCLUSIVE
//^

//> RANGEINCLUSIVE -> IMPLEMENTATION
impl<Type, const N: usize> const Index<RangeInclusive<usize>> for Array<Type, N> {
    type Output = [Type];
    #[inline]
    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {self.as_ref().index(index)}
}

//> RANGEINCLUSIVE -> MUT IMPLEMENTATION
impl<Type, const N: usize> const IndexMut<RangeInclusive<usize>> for Array<Type, N> {
    #[inline]
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {self.as_mut().index_mut(index)}
}


//^
//^ RANGETO
//^

//> RANGETO -> IMPLEMENTATION
impl<Type, const N: usize> const Index<RangeTo<usize>> for Array<Type, N> {
    type Output = [Type];
    #[inline]
    fn index(&self, index: RangeTo<usize>) -> &Self::Output {self.as_ref().index(index)}
}

//> RANGETO -> MUT IMPLEMENTATION
impl<Type, const N: usize> const IndexMut<RangeTo<usize>> for Array<Type, N> {
    #[inline]
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {self.as_mut().index_mut(index)}
}


//^
//^ RANGETOINCLUSIVE
//^

//> RANGETOINCLUSIVE -> IMPLEMENTATION
impl<Type, const N: usize> const Index<RangeToInclusive<usize>> for Array<Type, N> {
    type Output = [Type];
    #[inline]
    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {self.as_ref().index(index)}
}

//> RANGETOINCLUSIVE -> MUT IMPLEMENTATION
impl<Type, const N: usize> const IndexMut<RangeToInclusive<usize>> for Array<Type, N> {
    #[inline]
    fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut Self::Output {self.as_mut().index_mut(index)}
}