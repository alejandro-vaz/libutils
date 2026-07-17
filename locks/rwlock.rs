//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    users::Users,
    readguard::ReadGuard,
    writeguard::WriteGuard
};

//> HEAD -> CORE
use core::{
    cell::UnsafeCell, 
    mem::{
        forget,
        transmute_neo as transmute,
        replace
    }, 
    ops::DerefMut
};


//^
//^ RWLOCK
//^

//> RWLOCK -> STRUCT
pub struct RwLock<Type> {
    value: UnsafeCell<Type>,
    users: Users = Users {..}
}

//> RWLOCK -> IMPLEMENTATION
impl<Type> RwLock<Type> {
    pub const fn new(value: Type) -> Self {return Self {
        value: UnsafeCell::new(value),
        ..
    }}
    pub fn try_read<'valid>(&'valid self) -> Option<ReadGuard<'valid, Type>> {
        return Some(ReadGuard {
            reference: unsafe {self.value.as_ref_unchecked()},
            handle: self.users.read()?
        });
    }
    pub fn with_try_read<Return, Transformation: FnOnce(&Type) -> Return>(
        &self,
        function: Transformation
    ) -> Result<Return, Transformation> {return match self.try_read() {
        Some(readguard) => Ok(function(&readguard)),
        None => Err(function)
    }}
    pub fn read<'valid>(&'valid self) -> ReadGuard<'valid, Type> {
        return loop {match self.try_read() {
            Some(readguard) => break readguard,
            None => continue
        }}
    }
    pub fn with_read<Return, Transformation: FnOnce(&Type) -> Return>(
        &self, 
        function: Transformation
    ) -> Return {return function(&self.read())}
    pub fn try_write<'valid>(&'valid self) -> Option<WriteGuard<'valid, Type>> {
        return Some(WriteGuard {
            reference: unsafe {self.value.as_mut_unchecked()},
            handle: self.users.write()?
        });
    }
    pub fn with_try_write<Return, Transformation: FnOnce(&mut Type) -> Return>(
        &self,
        function: Transformation
    ) -> Result<Return, Transformation> {return match self.try_write() {
        None => Err(function),
        Some(mut writeguard) => Ok(function(&mut writeguard))
    }}
    pub fn write<'valid>(&'valid self) -> WriteGuard<'valid, Type> {
        return loop {match self.try_write() {
            Some(writeguard) => break writeguard,
            None => continue
        }}
    }
    pub fn with_write<Return, Transformation: FnOnce(&mut Type) -> Return>(
        &self,
        function: Transformation
    ) -> Return {return function(&mut self.write())}
    pub fn try_release(self) -> Result<Type, Self> {
        let (value, users) = unsafe {transmute::<_, (UnsafeCell<Type>, Users)>(self)};
        let maybewritehandle = users.write();
        let obtained = maybewritehandle.is_some();
        forget(maybewritehandle);
        return match obtained {
            false => Err(unsafe {transmute((value, users))}),
            true => Ok(value.into_inner())
        }
    }
    pub fn release(mut self) -> Type {return loop {match self.try_release() {
        Err(instance) => self = instance,
        Ok(value) => break value
    }}}
    pub fn try_replace(&self, with: Type) -> Result<Type, Type> {
        return Ok(replace(match self.try_write() {
            None => return Err(with),
            Some(writehandle) => writehandle
        }.deref_mut(), with));
    }
    pub fn replace(&self, mut with: Type) -> Type {return loop {match self.try_replace(with) {
        Err(failed) => with = failed,
        Ok(previous) => break previous
    }}}
    pub fn try_cloned(&self) -> Option<Type> where Type: Clone {
        return self.with_try_read(|value| value.clone()).ok();
    }
    pub fn cloned(&self) -> Type where Type: Clone {return loop {match self.try_cloned() {
        None => continue,
        Some(value) => break value
    }}}
}

//> RWLOCK -> DROP
impl<Type> Drop for RwLock<Type> {
    fn drop(&mut self) {forget(self.write())}
}

//> RWLOCK -> DEFAULT
const impl<Type: [const] Default> Default for RwLock<Type> {
    fn default() -> Self {return Self::new(Type::default())}
}

//> RWLOCK -> SEND
unsafe impl<Type> Send for RwLock<Type> {}

//> RWLOCK -> SYNC
unsafe impl<Type> Sync for RwLock<Type> {}