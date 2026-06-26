//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Log;

//> HEAD -> CORE
use core::borrow::Borrow;


//^
//^ REFERENCES
//^

//> REFERENCES -> SLICE
impl<Type> AsRef<[Type]> for Log<Type> {
    #[inline]
    fn as_ref(&self) -> &[Type] {return self.data.as_ref()}
}


//^
//^ BORROW
//^

//> BORROW -> CONSTANT
impl<Type> Borrow<[Type]> for Log<Type> {
    #[inline]
    fn borrow(&self) -> &[Type] {self.as_ref()}
}