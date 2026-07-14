//^
//^ HEAD
//^

//> HEAD -> HASHBROWN
use hashbrown::HashMap as Map;

//> HEAD -> SUPER
use super::policy::Policy;

//> HEAD -> CORE
use core::hash::Hash;


//^
//^ INFINITE
//^

//> INFINITE -> STRUCT
#[derive(Debug)]
pub struct InfiniteCache<Key: Clone + Eq + Hash, Value: Clone> {
    data: Map<Key, Value>
}

//> INFINITE -> POLICY
impl<Key: Clone + Eq + Hash, Value: Clone> Policy<Key, Value> for InfiniteCache<Key, Value> {
    fn len(&self) -> usize {return self.data.len()}
    fn is_full(&self) -> bool {return false}
    fn clear(&mut self) -> () {self.data.clear()}
    fn get<'valid>(&'valid mut self, key: Key, closure: impl FnOnce(Key) -> Value) -> Value {
        return match self.data.get(&key) {
            None => self.data.entry(key.clone()).or_insert(closure(key)).clone(),
            Some(value) => value.clone()
        }
    }
}

//> INFINITE -> DEFAULT
impl<Key: Clone + Eq + Hash, Value: Clone> Default for InfiniteCache<Key, Value> {
    fn default() -> Self {return Self {
        data: Map::new()
    }}
}

//> INFINITE -> IMPLEMENTATION
impl<Key: Clone + Eq + Hash, Value: Clone> InfiniteCache<Key, Value> {
    pub fn new() -> Self {return Self::default()}
}