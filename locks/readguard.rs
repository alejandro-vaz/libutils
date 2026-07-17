//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::readhandle::ReadHandle;

//> HEAD -> CORE
use core::ops::Deref;


//^
//^ READGUARD
//^

//> READGUARD -> STRUCT
pub struct ReadGuard<'valid, Type> {
    pub reference: &'valid Type,
    pub handle: ReadHandle<'valid>
}

//> READGUARD -> DEREF
impl<'valid, Type> Deref for ReadGuard<'valid, Type> {
    type Target = Type;
    fn deref(&self) -> &Self::Target {return self.reference}
}