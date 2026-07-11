//^
//^ HEAD
//^

//> HEAD -> CORE
use core::marker::Destruct;


//^
//^ UPDATE
//^

//> UPDATE -> TRAIT
#[must_use]
pub const trait Update: Sized + [const] Destruct {
    fn sync(self) -> ();
    fn ignore(self) -> () {}
}