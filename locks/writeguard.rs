//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::writehandle::WriteHandle;

//> HEAD -> CORE
use core::ops::{
    Deref,
    DerefMut
};


//^
//^ WRITEGUARD
//^

//> WRITEGUARD -> STRUCT
pub struct WriteGuard<'valid, Type> {
    pub reference: &'valid mut Type,
    pub handle: WriteHandle<'valid>
}

//> WRITEGUARD -> DEREF
impl<'valid, Type> Deref for WriteGuard<'valid, Type> {
    type Target = Type;
    fn deref(&self) -> &Self::Target {return self.reference}
}

//> WRITEGUARD -> DEREFMUT
impl<'valid, Type> DerefMut for WriteGuard<'valid, Type> {
    fn deref_mut(&mut self) -> &mut Self::Target {return self.reference}
}