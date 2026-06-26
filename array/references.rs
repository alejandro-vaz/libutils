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
impl<Type, const N: usize> const AsRef<[Type]> for Array<Type, N> {
    fn as_ref(&self) -> &[Type] {return unsafe {fat(self.data.as_ptr() as *const Type, self.length)}}
}

//> REFERENCES -> MUTABLE SLICE
impl<Type, const N: usize> const AsMut<[Type]> for Array<Type, N> {
    fn as_mut(&mut self) -> &mut [Type] {return unsafe {mutfat(self.pointer(), self.length)}}
}


//^
//^ BORROW
//^

//> BORROW -> CONSTANT
impl<Type, const N: usize> const Borrow<[Type]> for Array<Type, N> {
    fn borrow(&self) -> &[Type] {self.as_ref()}
}

//> BORROW -> MUTABLE
impl<Type, const N: usize> const BorrowMut<[Type]> for Array<Type, N> {
    fn borrow_mut(&mut self) -> &mut [Type] {self.as_mut()}
}