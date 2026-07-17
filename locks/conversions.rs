//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    mutex::Mutex,
    rwlock::RwLock
};


//^
//^ MUTEXFROM
//^

//> MUTEXFROM -> FUNCTION
const impl<Type, Generator: [const] FnOnce() -> Type> From<Generator> for Mutex<Type> {
    fn from(value: Generator) -> Self {return Self::new(value())}
}


//^
//^ RWLOCKFROM
//^

//> RWLOCKFROM -> FUNCTION
const impl<Type, Generator: [const] FnOnce() -> Type> From<Generator> for RwLock<Type> {
    fn from(value: Generator) -> Self {return Self::new(value())}
}