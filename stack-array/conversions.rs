//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    Array,
    errors::{
        UnmatchedCapacity,
        CapacityExceeded
    }
};

//> HEAD -> ALLOC
use alloc::vec::Vec;

//> HEAD -> CONSTRANGEITER
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
    Generator: [const] FnMut(usize) -> Type + [const] Destruct,
    const N: usize,
> From<Generator> for Array<Type, N> {
    fn from(mut value: Generator) -> Self {return Array {
        length: N,
        data: arrayfn(const |index| MaybeUninit::new(value(index)))
    }}
}

//> FROM -> VEC
const impl<Type, const N: usize> TryFrom<Vec<Type>> for Array<Type, N> {
    type Error = CapacityExceeded<N>;
    fn try_from(value: Vec<Type>) -> Result<Self, Self::Error> {
        let (pointer, length, _) = value.into_parts();
        if length > N {return Err(CapacityExceeded {
            expected: length
        })}
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

//> INTO -> FIXED ARRAY
const impl<
    Type, 
    const N: usize, 
    const LENGTH: usize
> TryInto<[Type; LENGTH]> for Array<Type, N> where [(); N - LENGTH]: {
    type Error = UnmatchedCapacity<LENGTH>;
    fn try_into(self) -> Result<[Type; LENGTH], Self::Error> {
        let (length, data) = Into::<(usize, [MaybeUninit<Type>; N])>::into(self);
        if length != LENGTH {return Err(UnmatchedCapacity {
            present: length
        })}
        let (initialized, _): (
            [MaybeUninit<Type>; LENGTH], 
            [MaybeUninit<Type>; N - LENGTH]
        ) = unsafe {transmute(data)};
        return Ok(unsafe {transmute(initialized)});
    }
}

//> INTO -> PARTS
const impl<Type, const N: usize> Into<(usize, [MaybeUninit<Type>; N])> for Array<Type, N> {
    fn into(self) -> (usize, [MaybeUninit<Type>; N]) {return unsafe {transmute(self)}}
}