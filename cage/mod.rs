//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(test)]
#![feature(const_trait_impl)]
#![feature(const_default)]
#![feature(lock_value_accessors)]

//> HEAD -> CRATES
extern crate test;

//> HEAD -> MODULES
#[cfg(test)]
mod benches;
mod derivations;
#[cfg(test)]
mod tests;

//> HEAD -> STD
use std::{ops::{Deref, DerefMut}, sync::{
    RwLock,
    RwLockReadGuard,
    RwLockWriteGuard
}};


//^
//^ CAGE
//^

//> CAGE -> DEFINITION
#[derive(Debug)]
pub struct Cage<Type: ?Sized> {
    being: RwLock<Type>
}

//> CAGE -> SIZED IMPLEMENTATION
impl<Type: Sized> Cage<Type> {
    #[inline]
    pub const fn new(value: Type) -> Cage<Type> {return Self {
        being: RwLock::new(value)
    }}
    #[inline]
    pub fn release(self) -> Type {return self.being.into_inner().unwrap()}
    #[inline]
    pub fn replace(&self, value: Type) -> Type {self.being.replace(value).unwrap()}
}

//> CAGE -> IMPLEMENTATION
impl<Type: ?Sized> Cage<Type> {
    #[inline]
    pub fn read<'valid>(&'valid self) -> RwLockReadGuard<'valid, Type> {return self.being.read().unwrap()}
    #[inline]
    pub fn write<'valid>(&'valid self) -> RwLockWriteGuard<'valid, Type> {return self.being.write().unwrap()}
    #[inline]
    pub fn with<Returns>(&self, closure: impl FnOnce(&Type) -> Returns) -> Returns {return closure(self.read().deref())}
    #[inline]
    pub fn with_mut<Returns>(&self, closure: impl FnOnce(&mut Type) -> Returns) -> Returns {return closure(self.write().deref_mut())}
}

//> CAGE -> COPY IMPLEMENTATION
impl<Type: Copy> Cage<Type> {
    #[inline]
    pub fn get(&self) -> Type {*self.read()}
}

//> CAGE -> CLONE IMPLEMENTATION
impl<Type: Clone> Cage<Type> {
    #[inline]
    pub fn cloned(&self) -> Type {self.read().clone()}
}

//> CAGE -> DEFAULT
const impl<Type: [const] Default> Default for Cage<Type> {
    #[inline]
    fn default() -> Self {return Self::new(Type::default())}
}