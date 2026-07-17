//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::exclusivehandle::ExclusiveHandle;

//> HEAD -> CORE
use core::ops::{
    Deref,
    DerefMut
};


//^
//^ EXCLUSIVEGUARD
//^

//> EXCLUSIVEGUARD -> STRUCT
pub struct ExclusiveGuard<'valid, Type> {
    pub reference: &'valid mut Type,
    pub handle: ExclusiveHandle<'valid>
}

//> EXCLUSIVEGUARD -> DEREF
impl<'valid, Type> Deref for ExclusiveGuard<'valid, Type> {
    type Target = Type;
    fn deref(&self) -> &Self::Target {return &self.reference}
}

//> EXCLUSIVEGUARD -> DEREFMUT
impl<'valid, Type> DerefMut for ExclusiveGuard<'valid, Type> {
    fn deref_mut(&mut self) -> &mut Self::Target {return &mut self.reference}
}
