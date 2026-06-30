//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;

//> HEAD -> ALLOC
use alloc::vec::Vec;


//^
//^ CONVERSIONS
//^

//> CONVERSIONS -> FROM FIXED TO VARIABLE
impl<Type, const M: usize, const N: usize> From<[Type; N]> for Array<Type, M> where [(); M - N]: {
    fn from(value: [Type; N]) -> Self {return Array::from_iter(value)}
}

//> CONVERSIONS -> TO VEC
impl<Type, const N: usize> Into<Vec<Type>> for Array<Type, N> {
    fn into(self) -> Vec<Type> {return Vec::from_iter(self.into_iter())}
}