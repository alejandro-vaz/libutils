//^
//^ HEAD
//^

//> HEAD -> SUPÈR
use super::Cage;

//> HEAD -> CORE
use core::{
    fmt::{
        Debug,
        Formatter,
        Result as Format
    },
    hash::{
        Hash,
        Hasher
    }
};


//^
//^ DERIVATIONS
//^

//> DERIVATIONS -> DEBUG
impl<Type: Debug + ?Sized> Debug for Cage<Type> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {write!(formatter, "Cage({:?})", &self.0)}
}

//> DERIVATIONS -> CLONE
impl<Type: Clone + ?Sized> Clone for Cage<Type> {
    fn clone(&self) -> Self {return Self::new(self.read().clone())}
}

//> DERIVATIONS -> HASH
impl<Type: Hash + ?Sized> Hash for Cage<Type> {
    fn hash<H: Hasher>(&self, state: &mut H) {return self.read().hash(state)}
}