//^
//^ HEAD
//^


//^
//^ ERRORS
//^

//> ERRORS -> UNMATCHED CAPACITY
pub struct UnmatchedCapacity<const EXPECTED: usize> {
    pub present: usize
}

//> ERRORS -> CAPACITY EXCEEDED
pub struct CapacityExceeded<const N: usize> {
    pub expected: usize
}