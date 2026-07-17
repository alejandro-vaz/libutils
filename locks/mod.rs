//^
//^ HEAD
//^

//> HEAD -> NO STD
#![no_std]

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> FEATURES
#![feature(const_trait_impl)]
#![feature(const_default)]
#![feature(transmute_neo)]
#![feature(unsafe_cell_access)]
#![feature(default_field_values)]
#![feature(const_convert)]
#![feature(generic_atomic)]

//> HEAD -> MODULES
mod conversions;
mod exclusiveguard;
mod exclusivehandle;
mod lock;
mod mutex;
mod readguard;
mod readhandle;
mod rwlock;
mod users;
mod writeguard;
mod writehandle;

//> HEAD -> MUTEX
pub use mutex::Mutex;

//> HEAD -> RWLOCK
pub use rwlock::RwLock;