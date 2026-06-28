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
const impl<Type: [const] PartialEq, const N: usize> PartialEq for Array<Type, N> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {self.as_ref() == other.as_ref()}
}

//> COMPARISONS -> ORD
const impl<Type: [const] PartialOrd, const N: usize> PartialOrd for Array<Type, N> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {self.as_ref().partial_cmp(other.as_ref())}
}

//> COMPARISONS -> HASH
impl<Type: Hash, const N: usize> Hash for Array<Type, N> {
    #[inline]
    fn hash<Hashing: Hasher>(&self, state: &mut Hashing) {Hash::hash(self.as_ref(), state)}
}

//> COMPARISONS -> TOTAL EQ
const impl<Type: [const] Eq, const N: usize> Eq for Array<Type, N> {}

//> COMPARISONS -> TOTAL ORD
const impl<Type: [const] Ord, const N: usize> Ord for Array<Type, N> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}