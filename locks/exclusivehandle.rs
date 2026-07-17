//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::lock::Lock;

//> HEAD -> CORE
use core::sync::atomic::Ordering;


//^
//^ EXCLUSIVEHANDLE
//^

//> EXCLUSIVEHANDLE -> STRUCT
pub struct ExclusiveHandle<'valid> {
    pub lock: &'valid Lock
}

//> EXCLUSIVEHANDLE -> DROP
impl<'valid> Drop for ExclusiveHandle<'valid> {
    fn drop(&mut self) {self.lock.lock.store(false, Ordering::Release)}
}