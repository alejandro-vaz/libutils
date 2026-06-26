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

//> CONVERSIONS -> BORROWED ARRAY
impl<'valid, Type, const N: usize> From<&'valid [Type; N]> for Log<&'valid Type> {
    fn from(value: &'valid [Type; N]) -> Self {return Self::from_iter(value)}
}

//> CONVERSIONS -> MUTABLE BORROWED ARRAY
impl<'valid, Type, const N: usize> From<&'valid mut [Type; N]> for Log<&'valid mut Type> {
    fn from(value: &'valid mut [Type; N]) -> Self {return Self::from_iter(value.into_iter())}
}

//> CONVERSIONS -> FROM SLICE
impl<'valid, Type> From<&'valid [Type]> for Log<&'valid Type> {
    fn from(value: &'valid [Type]) -> Self {return Self::from_iter(value.into_iter())}
}

//> CONVERSIONS -> FROM MUTABLE SLICE
impl<'valid, Type> From<&'valid mut [Type]> for Log<&'valid mut Type> {
    fn from(value: &'valid mut [Type]) -> Self {return Self::from_iter(value.into_iter())}
}