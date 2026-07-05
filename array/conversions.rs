//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;

//> HEAD -> ALLOC
use alloc::vec::Vec;

//> HEAD -> CORE
use core::{
    mem::{
        MaybeUninit,
        transmute_neo as transmute
    },
    ptr::copy_nonoverlapping,
    array::from_fn as arrayfn,
    marker::Destruct
};


//^
//^ FROM
//^

//> FROM -> FIXED ARRAY
const impl<Type, const M: usize, const N: usize> From<[Type; N]> for Array<Type, M> where [(); M - N]: {
    fn from(value: [Type; N]) -> Self {
        let mut array = Array {
            length: N,
            data: MaybeUninit::uninit()
        };
        unsafe {array.data.as_mut_ptr().cast::<[Type; N]>().write(value)};
        return array;
    }
}

//> FROM -> FUNCTION
const impl<
    Type: [const] Destruct, 
    const N: usize, 
    Generator: [const] FnMut(usize) -> Type + [const] Destruct
> From<Generator> for Array<Type, N> {
    fn from(value: Generator) -> Self {return Array {
        length: N,
        data: MaybeUninit::new(arrayfn(value))
    }}
}

//> FROM -> VEC
const impl<Type, const N: usize> TryFrom<Vec<Type>> for Array<Type, N> {
    type Error = &'static str;
    fn try_from(value: Vec<Type>) -> Result<Self, Self::Error> {
        let (pointer, length, _) = value.into_parts();
        if length > N {return Err("vector doesn't fit into array, length > N")}
        let mut array = Array {
            length: length,
            data: MaybeUninit::uninit()
        };
        unsafe {copy_nonoverlapping(pointer.as_ptr(), array.as_mut_ptr(), length)};
        return Ok(array);
    }
}

//> FROM -> PARTS
const impl<Type, const N: usize> From<(usize, MaybeUninit<[Type; N]>)> for Array<Type, N> {
    fn from(value: (usize, MaybeUninit<[Type; N]>)) -> Self {return unsafe {transmute(value)};}
}


//^
//^ INTO
//^

//> INTO -> VEC
impl<Type, const N: usize> Into<Vec<Type>> for Array<Type, N> {
    fn into(self) -> Vec<Type> {return Vec::from_iter(self.into_iter())}
}

//> INTO -> PARTS
const impl<Type, const N: usize> Into<(usize, MaybeUninit<[Type; N]>)> for Array<Type, N> {
    fn into(self) -> (usize, MaybeUninit<[Type; N]>) {return unsafe {transmute(self)}}
}