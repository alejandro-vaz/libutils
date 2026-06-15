//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;

//> HEAD -> CORE
use core::hash::{
    Hash,
    Hasher
};


//^
//^ COMPARISONS
//^

//> COMPARISONS -> EQ
impl<Type: PartialEq, const N: usize> PartialEq for Array<Type, N> {
    fn eq(&self, other: &Self) -> bool {self.as_ref() == other.as_ref()}
}

//> COMPARISONS -> ORD
impl<Type: PartialOrd, const N: usize> PartialOrd for Array<Type, N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {self.as_ref().partial_cmp(other.as_ref())}
}

//> COMPARISONS -> HASH
impl<Type: Hash, const N: usize> Hash for Array<Type, N> {
    fn hash<Hashing: Hasher>(&self, state: &mut Hashing) {Hash::hash(self.as_ref(), state)}
}

//> COMPARISONS -> TOTAL
impl<Type: Eq, const N: usize> Eq for Array<Type, N> {}
impl<Type: Ord, const N: usize> Ord for Array<Type, N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {self.as_ref().cmp(other.as_ref())}
}