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

//> CONVERSIONS -> FROM RAW
const impl<Type> From<*mut Type> for Pointer<Type> {
    fn from(value: *mut Type) -> Self {return Self {
        to: NonNull::new(value)
    }}
}

//> CONVERSIONS -> FROM REFERENCE
const impl<Type> From<&mut Type> for Pointer<Type> {
    fn from(value: &mut Type) -> Self {Self::from(value as *mut Type)}
}