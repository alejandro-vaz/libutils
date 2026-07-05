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
    pub fn ignore(self) -> () {}
    pub fn none<Wants>(self) -> Option<Wants> {return None}
    pub fn get(self) -> Option<Type> {return self.value}
}