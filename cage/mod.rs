//^
//^ HEAD
//^

//> HEAD -> MODULES
mod derivations;

//> HEAD -> STD
use std::sync::{
    RwLock,
    RwLockReadGuard,
    RwLockWriteGuard
};


//^
//^ CAGE
//^

//> CAGE -> DEFINITION
pub struct Cage<Type: ?Sized>(RwLock<Type>);

//> CAGE -> NEW
impl<Type: Sized> Cage<Type> {
    pub const fn new(value: Type) -> Cage<Type> {return Self(RwLock::new(value))}
    pub fn release(self) -> Type {return self.0.into_inner().unwrap()}
}

//> CAGE -> IMPLEMENTATION
impl<Type: ?Sized> Cage<Type> {
    #[inline]
    pub fn read<'valid>(&'valid self) -> RwLockReadGuard<'valid, Type> {return self.0.read().unwrap()}
    #[inline]
    pub fn write<'valid>(&'valid self) -> RwLockWriteGuard<'valid, Type> {return self.0.write().unwrap()}
}