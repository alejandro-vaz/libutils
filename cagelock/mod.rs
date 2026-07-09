//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(test)]
#![feature(const_trait_impl)]
#![feature(const_default)]
#![feature(const_convert)]
#![feature(lock_value_accessors)]

//> HEAD -> CRATES
extern crate test;

//> HEAD -> MODULES
#[cfg(test)]
mod benches;
mod conversions;
#[cfg(test)]
mod tests;

//> HEAD -> STD
use std::sync::RwLock;

//> HEAD -> CORE
use core::{
    ops::{
        Deref,
        DerefMut
    },
    hash::{
        Hash,
        Hasher
    }
};


//^
//^ CAGE
//^

//> CAGE -> DEFINITION
#[derive(Debug)]
pub struct Cage<Type: ?Sized> {
    being: RwLock<Type>
}

//> CAGE -> IMPLEMENTATION
impl<Type: ?Sized> Cage<Type> {
    pub fn read<Returns>(&self, closure: impl FnOnce(&Type) -> Returns) -> Returns {
        return closure(self.being.read().unwrap().deref())
    }
    pub fn write<Returns>(&self, closure: impl FnOnce(&mut Type) -> Returns) -> Returns {
        return closure(self.being.write().unwrap().deref_mut())
    }
    pub const fn new(value: Type) -> Cage<Type> where Type: Sized {return Self {
        being: RwLock::new(value)
    }}
    pub fn release(self) -> Type where Type: Sized {return self.being.into_inner().unwrap()}
    pub fn replace(&self, value: Type) -> Type where Type: Sized {self.being.replace(value).unwrap()}
    pub fn cloned(&self) -> Type where Type: Clone {self.being.read().unwrap().clone()}
    pub fn get(&self) -> Type where Type: Copy {*self.being.read().unwrap()}
}

//> CAGE -> DEFAULT
const impl<Type: [const] Default> Default for Cage<Type> {
    fn default() -> Self {return Self::new(Type::default())}
}

//> CAGE -> CLONE
impl<Type: Clone + ?Sized> Clone for Cage<Type> {
    fn clone(&self) -> Self {return Self::new(self.read(|value| value.clone()))}
}

//> CAGE -> HASH
impl<Type: Hash + ?Sized> Hash for Cage<Type> {
    fn hash<H: Hasher>(&self, state: &mut H) {return self.read(|value| value.hash(state))}
}