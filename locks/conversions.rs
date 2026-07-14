//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Mutex;


//^
//^ FROM
//^

//> FROM -> FUNCTION
const impl<Type, Generator: [const] FnOnce() -> Type> From<Generator> for Mutex<Type> {
    fn from(value: Generator) -> Self {return Self::new(value())}
}