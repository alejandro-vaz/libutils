//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;


//^
//^ CONVERSIONS
//^

//> CONVERSIONS -> FROM FIXED TO VARIABLE
impl<Type, const M: usize, const N: usize> From<[Type; N]> for Array<Type, M> {
    fn from(value: [Type; N]) -> Self {
        let mut array = Array::new();
        array.extend(value);
        return array;
    }
}