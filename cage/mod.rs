//^
//^ HEAD
//^

//> HEAD -> STD
use std::sync::{
    RwLock,
    RwLockReadGuard,
    RwLockWriteGuard
};

//> HEAD -> CORE
use core::fmt::{
    Debug,
    Formatter,
    Result as Format
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
    pub fn read(&self) -> RwLockReadGuard<'_, Type> {return self.0.read().unwrap()}
    #[inline]
    pub fn write(&self) -> RwLockWriteGuard<'_, Type> {return self.0.write().unwrap()}
}

//> CAGE -> DEBUG
impl<Type: Debug + ?Sized> Debug for Cage<Type> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {write!(formatter, "Cage({:?})", &self.0)}
}