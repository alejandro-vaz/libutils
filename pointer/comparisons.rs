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
impl<'valid, Type> PartialEq for Pointer<'valid, Type> {
    fn eq(&self, other: &Self) -> bool {return self.as_ptr().eq(&other.as_ptr())}
}

//> POINTER -> PARTIAL ORD
impl<'valid, Type> PartialOrd for Pointer<'valid, Type> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {return self.as_ptr().partial_cmp(&other.as_ptr())}
}

//> POINTER -> EQ
impl<'valid, Type> Eq for Pointer<'valid, Type> {}

//> POINTER -> ORD
impl<'valid, Type> Ord for Pointer<'valid, Type> {
    fn cmp(&self, other: &Self) -> Ordering {return self.as_ptr().cmp(&other.as_ptr())}
}

//> POINTER -> HASH
impl<'valid, Type> Hash for Pointer<'valid, Type> {
    fn hash<H: Hasher>(&self, state: &mut H) {return Hash::hash(&self.as_ptr(), state)}
}