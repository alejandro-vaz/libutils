//^
//^ HEAD
//^

//> HEAD -> CORE
use core::sync::atomic::{
    Atomic,
    Ordering
};

//> HEAD -> SUPER
use super::exclusivehandle::ExclusiveHandle;


//^
//^ LOCK
//^

//> LOCK -> STRUCT
#[derive(Debug)]
pub struct Lock {
    pub lock: Atomic<bool> = Atomic::<bool>::new(false)
}

//> LOCK -> IMPLEMENTATION
impl Lock {
    pub fn acquire<'valid>(&'valid self) -> Option<ExclusiveHandle<'valid>> {
        self.lock.compare_exchange(
            false, 
            true, 
            Ordering::AcqRel, 
            Ordering::Acquire
        ).ok()?;
        return Some(ExclusiveHandle {
            lock: self
        })
    }
}