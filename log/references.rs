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
const impl<Type> AsRef<[Type]> for Log<Type> {
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
const impl<Type> Borrow<[Type]> for Log<Type> {
    #[inline]
    fn borrow(&self) -> &[Type] {self.as_ref()}
}