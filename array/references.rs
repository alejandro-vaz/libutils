//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;

//> HEAD -> CORE
use core::{
    slice::{
        from_raw_parts as fat,
        from_raw_parts_mut as mutfat
    },
    borrow::{
        Borrow,
        BorrowMut
    }
};


//^
//^ REFERENCES
//^

//> REFERENCES -> SLICE
const impl<Type, const N: usize> AsRef<[Type]> for Array<Type, N> {
    #[inline]
    fn as_ref(&self) -> &[Type] {return unsafe {fat(self.data.as_ptr() as *const Type, self.length)}}
}

//> REFERENCES -> MUTABLE SLICE
const impl<Type, const N: usize> AsMut<[Type]> for Array<Type, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [Type] {return unsafe {mutfat(self.pointer().as_ptr(), self.length)}}
}


//^
//^ BORROW
//^

//> BORROW -> CONSTANT
const impl<Type, const N: usize> Borrow<[Type]> for Array<Type, N> {
    #[inline]
    fn borrow(&self) -> &[Type] {self.as_ref()}
}

//> BORROW -> MUTABLE
const impl<Type, const N: usize> BorrowMut<[Type]> for Array<Type, N> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [Type] {self.as_mut()}
}