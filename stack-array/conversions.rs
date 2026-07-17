//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;

//> HEAD -> ALLOC
use alloc::vec::Vec;
use constrangeiter::ConstIntoIterator;

//> HEAD -> CORE
use core::{
    mem::{
        MaybeUninit,
        transmute_neo as transmute
    },
    array::from_fn as arrayfn,
    marker::Destruct
};


//^
//^ FROM
//^

//> FROM -> FIXED ARRAY
const impl<
    Type: [const] Destruct, 
    const M: usize, 
    const N: usize
> From<[Type; N]> for Array<Type, M> where [(); M - N]: {
    fn from(value: [Type; N]) -> Self {
        let mut data = MaybeUninit::<[Type; M]>::uninit().transpose();
        let mut index = 0;
        let _ = value.map(const |element| {
            data[index].write(element); 
            index += 1;
        });
        return Array {
            length: N,
            data: data
        };
    }
}

//> FROM -> FUNCTION
const impl<
    Type: [const] Destruct, 
    const N: usize, 
    Generator: [const] FnMut(usize) -> Type + [const] Destruct
> From<Generator> for Array<Type, N> {
    fn from(mut value: Generator) -> Self {return Array {
        length: N,
        data: arrayfn(const |index| MaybeUninit::new(value(index)))
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
            data: MaybeUninit::uninit().transpose()
        };
        for index in (0..length).const_into_iter() {
            array.data[index].write(unsafe {pointer.add(index).read()});
        }
        return Ok(array);
    }
}

//> FROM -> PARTS
const impl<Type, const N: usize> From<(usize, [MaybeUninit<Type>; N])> for Array<Type, N> {
    fn from(value: (usize, [MaybeUninit<Type>; N])) -> Self {return unsafe {transmute(value)}}
}


//^
//^ INTO
//^

//> INTO -> VEC
impl<Type, const N: usize> Into<Vec<Type>> for Array<Type, N> {
    fn into(self) -> Vec<Type> {return Vec::from_iter(self.into_iter())}
}

//> INTO -> PARTS
const impl<Type, const N: usize> Into<(usize, [MaybeUninit<Type>; N])> for Array<Type, N> {
    fn into(self) -> (usize, [MaybeUninit<Type>; N]) {return unsafe {transmute(self)}}
}