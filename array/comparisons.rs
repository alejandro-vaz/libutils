//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;


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

//> COMPARISONS -> TOTAL
impl<Type: Eq, const N: usize> Eq for Array<Type, N> {}
impl<Type: Ord, const N: usize> Ord for Array<Type, N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {self.as_ref().cmp(other.as_ref())}
}