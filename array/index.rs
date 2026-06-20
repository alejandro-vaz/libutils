//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;

//> HEAD -> CORE
use core::ops::{
    Index,
    IndexMut
};


//^
//^ INDEX
//^

//> INDEX -> IMPLEMENTATION
impl<Type, const N: usize> Index<usize> for Array<Type, N> {
    type Output = Type;
    fn index(&self, index: usize) -> &Self::Output {return self.get(index).unwrap()}
}

//> INDEX -> MUT
impl<Type, const N: usize> IndexMut<usize> for Array<Type, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {return self.get_mut(index).unwrap()}
}