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
    pub fn get<Return>(&self, function: impl FnOnce(&mut Type) -> Return) -> Return {
        let handle = Handle::acquire(&self.lock);
        let value = function(unsafe {self.value.get().as_mut().unwrap()});
        drop(handle);
        return value;
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
        let clone = unsafe {self.value.get().as_ref().unwrap().clone()};
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