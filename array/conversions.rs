//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;

//> HEAD -> CORE
use core::mem::MaybeUninit;


//^
//^ CONVERSIONS
//^

//> CONVERSIONS -> FROM FIXED ARRAY
impl<Type, const M: usize, const N: usize> From<[Type; N]> for Array<Type, M> where [(); M - N]: {
    fn from(value: [Type; N]) -> Self {return unsafe {Self::arranged(N, MaybeUninit::new((&value as *const Type as *const [Type; M]).read()))}}
}