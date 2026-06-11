//^
//^ HEAD
//^

//> HEAD -> CORE
use core::ops::DerefMut;

//> HEAD -> STD
use std::sync::Mutex;


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
    pub fn free<Return, Closure: FnOnce(&mut Type) -> Return>(&self, closure: Closure) -> Return {
        return closure(self.0.lock().unwrap().deref_mut());
    }
}

//> CAGE -> SNAPSHOT
impl<Type: Clone> Cage<Type> {
    pub fn snapshot(&self) -> Type {return self.0.lock().unwrap().clone()}
}