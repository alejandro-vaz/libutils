//^
//^ HANDLE
//^

//> HANDLE -> TRAIT
#[must_use]
pub trait Handle {
    fn sync(self) -> ();
    fn ignore(self) -> ();
}