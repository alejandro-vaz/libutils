//^
//^ HEAD
//^

//> HEAD -> HASHBROWN
use hashbrown::HashMap as Map;

//> HEAD -> CORE
use core::hash::Hash;

//> HEAD -> SUPER
use super::policy::Policy;


//^
//^ LRU
//^

//> LRU -> STRUCT
#[derive(Debug)]
pub struct LruCache<Key: Clone + Hash + Eq, Value: Clone> {
    first: Option<(Key, Value, Option<Key>)>, // key - value - previous
    body: Map<Key, (Value, Option<Key>, Key)>, // key - value - previous - next
    last: Option<(Key, Value, Key)>, // key - value - next
    capacity: usize
}

//> LRU -> POLICY
impl<Key: Clone + Hash + Eq, Value: Clone> Policy<Key, Value> for LruCache<Key, Value> {
    fn len(&self) -> usize {
        return self.first.is_some() as usize + self.last.is_some() as usize + self.body.len();
    }
    fn is_full(&self) -> bool {return {
        self.first.is_some() 
        && self.last.is_some() 
        && self.body.len() == self.capacity.wrapping_sub(2)
    }}
    fn clear(&mut self) -> () {
        self.first.take();
        self.last.take();
        self.body.clear();
    }
    fn get<'valid>(&'valid mut self, key: Key, closure: impl FnOnce(Key) -> Value) -> Value {
        return match self.capacity {
            0 => closure(key),
            1 => match &mut self.first {
                Some((index, value, _)) => match *index == key {
                    false => {
                        *index = key.clone();
                        *value = closure(key);
                        value.clone()
                    },
                    true => value.clone()
                },
                None => {
                    let value = closure(key.clone());
                    self.first = Some((key, value.clone(), None));
                    value
                }
            },
            2.. => unreachable!()
        }
    }
}

//> LRU -> IMPLEMENTATION
impl<Key: Clone + Hash + Eq, Value: Clone> LruCache<Key, Value> {
    pub fn new(capacity: usize) -> Self {return Self {
        first: None,
        last: None,
        capacity: capacity,
        body: Map::with_capacity(capacity.saturating_sub(2))
    }}
}