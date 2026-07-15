//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::handle::Handle;
//> HEAD -> CORE
use core::ops::{
    Deref,
    DerefMut
};


//^
//^ GUARD
//^

//> GUARD -> STRUCT
pub struct Guard<'valid, Type> {
    pub reference: &'valid mut Type,
    pub handle: Handle<'valid>
}

//> GUARD -> DEREF
impl<'valid, Type> Deref for Guard<'valid, Type> {
    type Target = Type;
    fn deref(&self) -> &Self::Target {return &self.reference}
}

//> GUARD -> DEREFMUT
impl<'valid, Type> DerefMut for Guard<'valid, Type> {
    fn deref_mut(&mut self) -> &mut Self::Target {return &mut self.reference}
}
