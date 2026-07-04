//^
//^ THEN
//^

//> THEN -> STRUCT
pub struct Then;

//> THEN -> IMPLEMENTATION
impl Then {
    #[inline]
    pub fn ignore(self) -> () {}
    #[inline]
    pub fn none<Type>(self) -> Option<Type> {return None}
}