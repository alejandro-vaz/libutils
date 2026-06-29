//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Pointer;

//> HEAD -> CORE
use core::{
    cmp::Ordering,
    hash::{
        Hash,
        Hasher
    }
};


//^
//^ COMPARISONS
//^

//> POINTER -> PARTIAL EQ
impl<Type> PartialEq for Pointer<Type> {
    fn eq(&self, other: &Self) -> bool {return self.as_ptr().eq(&other.as_ptr())}
}

//> POINTER -> PARTIAL ORD
impl<Type> PartialOrd for Pointer<Type> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {return self.as_ptr().partial_cmp(&other.as_ptr())}
}

//> POINTER -> EQ
impl<Type> Eq for Pointer<Type> {}

//> POINTER -> ORD
impl<Type> Ord for Pointer<Type> {
    fn cmp(&self, other: &Self) -> Ordering {return self.as_ptr().cmp(&other.as_ptr())}
}

//> POINTER -> HASH
impl<Type> Hash for Pointer<Type> {
    fn hash<H: Hasher>(&self, state: &mut H) {return Hash::hash(&self.as_ptr(), state)}
}