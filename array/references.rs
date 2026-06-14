//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;

//> HEAD -> CORE
use core::slice::{
    from_raw_parts as fat,
    from_raw_parts_mut as mutfat
};


//^
//^ REFERENCES
//^

//> REFERENCES -> SLICE
impl<Type, const N: usize> AsRef<[Type]> for Array<Type, N> where [(); size_of::<Type>() * N]: {
    fn as_ref(&self) -> &[Type] {return unsafe{fat(self.ptr(), self.length)}}
}

//> REFERENCES -> MUTABLE SLICE
impl<Type, const N: usize> AsMut<[Type]> for Array<Type, N> where [(); size_of::<Type>() * N]: {
    fn as_mut(&mut self) -> &mut [Type] {return unsafe {mutfat(self.mutptr(), self.length)}}
}