//^
//^ METADATA
//^

//> METADATA -> TRAIT
#[must_use]
pub const trait Metadata: Sized {
    fn size(&self) -> usize;
}