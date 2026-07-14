//^
//^ HEAD
//^

//> HEAD -> NO STD
#![no_std]

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> FEATURES
#![feature(const_trait_impl)]

//> HEAD -> MODULES
mod infinite;
mod lru;
mod policy;

//> HEAD -> INFINITE
pub use infinite::InfiniteCache;

//> HEAD -> LRU
pub use lru::LruCache;

//> HEAD -> POLICY
pub use policy::Policy;