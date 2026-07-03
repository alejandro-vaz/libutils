//^
//^ HEAD
//^

//> HEAD -> SUPÈR
use super::Cage;

//> HEAD -> CORE
use core::hash::{
    Hash,
    Hasher
};


//^
//^ DERIVATIONS
//^

//> DERIVATIONS -> CLONE
impl<Type: Clone + ?Sized> Clone for Cage<Type> {
    fn clone(&self) -> Self {return Self::new(self.read(|value| value.clone()))}
}

//> DERIVATIONS -> HASH
impl<Type: Hash + ?Sized> Hash for Cage<Type> {
    fn hash<H: Hasher>(&self, state: &mut H) {return self.read(|value| value.hash(state))}
}