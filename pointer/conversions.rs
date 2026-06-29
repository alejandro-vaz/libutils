//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Pointer;

//> HEAD -> CORE
use core::ptr::NonNull;


//^
//^ CONVERSIONS
//^

//> CONVERSIONS -> FROM NONNULL
const impl<Type> From<NonNull<Type>> for Pointer<Type> {
    fn from(value: NonNull<Type>) -> Self {return Self {
        to: Some(value)
    }}
}

//> CONVERSIONS -> FROM OPTION NONNULL
const impl<Type> From<Option<NonNull<Type>>> for Pointer<Type> {
    fn from(value: Option<NonNull<Type>>) -> Self {return Self {
        to: value
    }}
}