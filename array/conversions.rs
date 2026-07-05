//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;

//> HEAD -> ALLOC
use alloc::vec::Vec;


//^
//^ FROM
//^

//> FROM -> FIXED TO VARIABLE
impl<Type, const M: usize, const N: usize> From<[Type; N]> for Array<Type, M> where [(); M - N]: {
    fn from(value: [Type; N]) -> Self {return Array::from_iter(value)}
}

//> FROM -> FUNCTION
impl<Type, const N: usize, Generator: FnMut(usize) -> Type> From<Generator> for Array<Type, N> {
    fn from(mut value: Generator) -> Self {
        let mut array = Self::new();
        for index in 0..N {
            array.push(value(index));
        };
        return array;
    }
}

//> FROM -> VEC
impl<Type, const N: usize> TryFrom<Vec<Type>> for Array<Type, N> {
    type Error = &'static str;
    fn try_from(value: Vec<Type>) -> Result<Self, Self::Error> {return if value.len() > N {
        Err("not enough capacity in array")
    } else {Ok(Self::from_iter(value.into_iter()))}}
}


//^
//^ INTO
//^

//> INTO -> VEC
impl<Type, const N: usize> Into<Vec<Type>> for Array<Type, N> {
    fn into(self) -> Vec<Type> {return Vec::from_iter(self.into_iter())}
}