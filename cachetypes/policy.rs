//^
//^ POLICY
//^

//> POLICY -> TRAIT
pub const trait Policy<Key: Clone, Value: Clone> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {return self.len() == 0}
    fn is_full(&self) -> bool;
    fn clear(&mut self) -> ();
    fn get<'valid>(&'valid mut self, key: Key, closure: impl FnOnce(Key) -> Value) -> Value;
}