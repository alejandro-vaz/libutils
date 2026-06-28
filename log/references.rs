//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Log;

//> HEAD -> CORE
use core::{
    borrow::Borrow,
    slice::from_raw_parts as fat
};


//^
//^ REFERENCES
//^

//> REFERENCES -> SLICE
impl<Type> const AsRef<[Type]> for Log<Type> {
    #[inline]
    fn as_ref(&self) -> &[Type] {return match self.pointer.take() {
        None => &[],
        Some(pointer) => unsafe {fat(pointer.as_ptr(), self.length)}
    }}
}


//^
//^ BORROW
//^

//> BORROW -> CONSTANT
impl<Type> const Borrow<[Type]> for Log<Type> {
    #[inline]
    fn borrow(&self) -> &[Type] {self.as_ref()}
}