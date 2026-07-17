//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    lock::Lock,
    exclusiveguard::ExclusiveGuard
};

//> HEAD -> CORE
use core::{
    cell::UnsafeCell,
    mem::{
        forget,
        transmute_neo as transmute,
        replace
    }
};


//^
//^ MUTEX
//^

//> MUTEX -> STRUCT
#[derive(Debug)]
pub struct Mutex<Type> {
    value: UnsafeCell<Type>,
    lock: Lock = Lock {..}
}

//> MUTEX -> IMPLEMENTATION
impl<Type> Mutex<Type> {
    pub const fn new(value: Type) -> Self {return Self {
        value: UnsafeCell::new(value),
        ..
    }}
    pub fn try_with<Return, Transformation: FnOnce(&mut Type) -> Return>(
        &self, 
        function: Transformation
    ) -> Result<Return, Transformation> {
        let mut exclusiveguard = match self.try_get() {
            Some(exclusiveguard) => exclusiveguard,
            None => return Err(function)
        };
        return Ok(function(&mut exclusiveguard));
    }
    pub fn with<Return>(&self, mut function: impl FnOnce(&mut Type) -> Return) -> Return {
        return loop {match self.try_with(function) {
            Err(given) => function = given,
            Ok(value) => break value
        }}
    }
    pub fn try_get<'valid>(&'valid self) -> Option<ExclusiveGuard<'valid, Type>> {
        return Some(ExclusiveGuard {
            reference: unsafe {self.value.as_mut_unchecked()},
            handle: self.lock.acquire()?
        }
    )}
    pub fn get<'valid>(&'valid self) -> ExclusiveGuard<'valid, Type> {return loop {
        match self.try_get() {
            None => continue,
            Some(exclusiveguard) => break exclusiveguard
        }
    }}
    pub fn try_release(self) -> Result<Type, Self> {
        let (value, lock) = unsafe {transmute::<_, (UnsafeCell<Type>, Lock)>(self)};
        let maybeexclusivehandle = lock.acquire();
        let obtained = maybeexclusivehandle.is_some();
        forget(maybeexclusivehandle);
        return match obtained {
            true => Ok(value.into_inner()),
            false => Err(unsafe {transmute((value, lock))})
        }
    }
    pub fn release(mut self) -> Type {return loop {match self.try_release() {
        Err(instance) => self = instance,
        Ok(value) => break value
    }}}
    pub fn try_replace(&self, with: Type) -> Result<Type, Type> {
        let mut exclusivehandle = match self.try_get() {
            Some(exclusivehandle) => exclusivehandle,
            None => return Err(with)
        };
        return Ok(replace(&mut exclusivehandle, with));
    }
    pub fn replace(&self, mut with: Type) -> Type {return loop {match self.try_replace(with) {
        Ok(previous) => break previous,
        Err(failed) => with = failed
    }}}
    pub fn try_cloned(&self) -> Option<Type> where Type: Clone {
        return self.try_with(|value| value.clone()).ok();
    }
    pub fn cloned(&self) -> Type where Type: Clone {return loop {match self.try_cloned() {
        None => continue,
        Some(clone) => break clone
    }}}
}

//> MUTEX -> DROP
impl<Type> Drop for Mutex<Type> {
    fn drop(&mut self) {forget(self.get())}
}

//> MUTEX -> DEFAULT
const impl<Type: [const] Default> Default for Mutex<Type> {
    fn default() -> Self {return Self::new(Type::default())}
}

//> MUTEX -> SEND
unsafe impl<Type> Send for Mutex<Type> {}

//> MUTEX -> SYNC
unsafe impl<Type> Sync for Mutex<Type> {}
