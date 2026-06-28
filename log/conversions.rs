//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Log;


//^
//^ CONVERSIONS
//^

//> CONVERSIONS -> FROM ARRAY
impl<Type, const N: usize> From<[Type; N]> for Log<Type> {
    fn from(value: [Type; N]) -> Self {return Self::from_iter(value)}
}