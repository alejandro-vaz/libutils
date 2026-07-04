//^
//^ THEN
//^

//> THEN -> STRUCT
#[must_use]
pub struct Then<Type> {
    pub value: Option<Type>
}

//> THEN -> IMPLEMENTATION
impl<Type> Then<Type> {
    #[inline]
    pub fn ignore(self) -> () {}
    #[inline]
    pub fn none<Wants>(self) -> Option<Wants> {return None}
    #[inline]
    pub fn get(self) -> Option<Type> {return self.value}
}