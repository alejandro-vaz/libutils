//^
//^ HEAD
//^

//> HEAD -> CORE
use core::ops::DerefMut;

//> HEAD -> STD
use std::{
    sync::{
        Mutex,
        MutexGuard
    },
    fmt::{
        Debug,
        Formatter,
        Result as Format
    }
};


//^
//^ CAGE
//^

//> CAGE -> DEFINITION
pub struct Cage<Type: ?Sized>(Mutex<Type>);

//> CAGE -> NEW
impl<Type: Sized> Cage<Type> {
    pub const fn new(value: Type) -> Cage<Type> {return Self(Mutex::new(value))}
    pub fn release(self) -> Type {return self.0.into_inner().unwrap()}
}

//> CAGE -> IMPLEMENTATION
impl<Type: ?Sized> Cage<Type> {
    #[inline]
    pub fn peak<Return, Closure: FnOnce(&mut Type) -> Return>(&self, closure: Closure) -> Return {
        return closure(self.0.lock().unwrap().deref_mut());
    }
    #[inline]
    pub fn free(&self) -> MutexGuard<'_, Type> {return self.0.lock().unwrap()}
}

//> CAGE -> CLONED
impl<Type: Clone> Cage<Type> {
    pub fn cloned(&self) -> Type {return self.0.lock().unwrap().clone()}
}

//> CAGE -> DEBUG
impl<Type: Debug + ?Sized> Debug for Cage<Type> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {
        write!(formatter, "Cage({:?})", &self.0)
    }
}