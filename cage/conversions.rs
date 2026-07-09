//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Cage;


//^
//^ FROM
//^

//> FROM -> FUNCTION
const impl<Type, Generator: [const] FnOnce() -> Type> From<Generator> for Cage<Type> {
    fn from(value: Generator) -> Self {return Self::new(value())}
}