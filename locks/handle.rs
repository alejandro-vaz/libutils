//^
//^ HEAD
//^

//> HEAD -> CORE
use core::sync::atomic::{
    Atomic,
    Ordering
};


//^
//^ HANDLE
//^

//> HANDLE -> STRUCT
pub struct Handle<'valid> {
    pub lock: &'valid Atomic<bool>
}

//> HANDLE -> IMPLEMENTATION
impl<'valid> Handle<'valid> {
    pub fn acquire(lock: &'valid Atomic<bool>) -> Self {
        while lock.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Acquire).is_err() {}
        return Self {lock: lock};
    }
    pub fn acquire_now(lock: &'valid Atomic<bool>) -> Option<Self> {
        return lock.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Acquire).ok().map(|_| Self {lock: lock});
    }
}

//> HANDLE -> DROP
impl<'valid> Drop for Handle<'valid> {
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release)
    }
}
