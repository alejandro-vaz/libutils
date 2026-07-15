//^
//^ HEAD
//^

//> HEAD -> NO STD
#![no_std]

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> FEATURES
#![feature(const_trait_impl)]
#![feature(const_default)]
#![feature(transmute_neo)]
#![feature(unsafe_cell_access)]
#![feature(default_field_values)]
#![feature(const_convert)]
#![feature(generic_atomic)]

//> HEAD -> MODULES
mod conversions;
mod guard;
mod handle;

//> HEAD -> CORE
use core::{
    sync::atomic::Atomic,
    mem::{
        transmute_neo as transmute,
        forget
    },
    hash::{
        Hash,
        Hasher
    },
    cell::UnsafeCell
};

//> HEAD -> HANDLE
use handle::Handle;

//> HEAD -> GUARD
use guard::Guard;


//^
//^ MUTEX
//^

//> MUTEX -> STRUCT
#[derive(Debug)]
pub struct Mutex<Type> {
    value: UnsafeCell<Type>,
    lock: Atomic<bool> = Atomic::<bool>::new(false)
}

//> MUTEX -> IMPLEMENTATION
impl<Type> Mutex<Type> {
    pub const fn new(value: Type) -> Self {return Self {
        value: UnsafeCell::new(value),
        ..
    }}
    pub fn try_with<Return>(&self, function: impl FnOnce(&mut Type) -> Return) -> Option<Return> {
        let handle = Handle::acquire_now(&self.lock)?;
        let value = function(unsafe {self.value.get().as_mut_unchecked()});
        drop(handle);
        return Some(value);
    }
    pub fn with<Return>(&self, function: impl FnOnce(&mut Type) -> Return) -> Return {
        let handle = Handle::acquire(&self.lock);
        let value = function(unsafe {self.value.get().as_mut_unchecked()});
        drop(handle);
        return value;
    }
    pub fn try_get<'any, 'valid>(&'valid self) -> Option<Guard<'any, Type>> where 'valid: 'any {
        return Some(Guard {
            reference: unsafe {self.value.get().as_mut_unchecked()},
            handle: Handle::acquire_now(&self.lock)?
        })
    }
    pub fn get<'any, 'valid>(&'valid self) -> Guard<'any, Type> where 'valid: 'any {
        return Guard {
            reference: unsafe {self.value.get().as_mut_unchecked()},
            handle: Handle::acquire(&self.lock)
        }
    }
    pub fn release(self) -> Type {
        let (value, lock) = unsafe {transmute::<_, (Type, Atomic<bool>)>(self)};
        Handle::acquire(&lock);
        return value;
    }
    pub fn replace(&self, with: Type) -> Type {
        let handle = Handle::acquire(&self.lock);
        let previous = unsafe {self.value.replace(with)};
        drop(handle);
        return previous;
    }
    pub fn cloned(&self) -> Type where Type: Clone {
        let handle = Handle::acquire(&self.lock);
        let clone = unsafe {self.value.get().as_ref_unchecked().clone()};
        drop(handle);
        return clone;
    }
}

//> MUTEX -> DROP
impl<Type> Drop for Mutex<Type> {
    fn drop(&mut self) {forget(Handle::acquire(&self.lock))}
}

//> MUTEX -> DEFAULT
const impl<Type: [const] Default> Default for Mutex<Type> {
    fn default() -> Self {return Self::new(Type::default())}
}

//> MUTEX -> CLONE
impl<Type: Clone> Clone for Mutex<Type> {
    fn clone(&self) -> Self {return Self {
        value: UnsafeCell::new(self.cloned()),
        ..
    }}
}

//> MUTEX -> HASH
impl<Type: Hash> Hash for Mutex<Type> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let handle = Handle::acquire(&self.lock);
        unsafe {self.value.get().as_ref().unwrap().hash(state)};
        drop(handle);
    }
}

//> MUTEX -> SEND
unsafe impl<Type: Send> Send for Mutex<Type> {}

//> MUTEX -> SYNC
unsafe impl<Type: Sync> Sync for Mutex<Type> {}
