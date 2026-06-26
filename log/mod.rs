//^
//^ HEAD
//^

//> HEAD -> MODULES
mod conversions;
mod iterators;
mod references;

//> HEAD -> STD
use core::fmt::{
    Debug,
    Formatter,
    Result as Format
};


//^
//^ LOG
//^

//> LOG -> STRUCT
#[derive(Clone)]
pub struct Log<Type> {
    data: Vec<Type>
}

//> LOG -> IMPLEMENTATION
impl<Type> Log<Type> {
    #[inline]
    pub const fn new() -> Self {return Self::default()}
    #[inline]
    pub const fn len(&self) -> usize {return self.data.len()}
    #[inline]
    pub fn push(&mut self, value: Type) -> () {self.data.push(value)}
}

//> LOG -> DEFAULT
impl<Type> const Default for Log<Type> {
    #[inline]
    fn default() -> Self {return Self {
        data: Vec::new()
    }}
}

//> LOG -> EXTEND
impl<Type> Extend<Type> for Log<Type> {
    fn extend<T: IntoIterator<Item = Type>>(&mut self, iter: T) {self.data.extend(iter)}
}

//> LOG -> DEBUG
impl<Type: Debug> Debug for Log<Type> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {Debug::fmt(self.as_ref(), formatter)}
}