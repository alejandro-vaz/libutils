//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Pointer;

//> HEAD -> CORE
use core::{
    ptr::NonNull,
    marker::PhantomCovariantLifetime
};


//^
//^ CONVERSIONS
//^

//> CONVERSIONS -> FROM NONNULL
const impl<Type> From<NonNull<Type>> for Pointer<'static, Type> {
    fn from(value: NonNull<Type>) -> Self {return Self {
        to: Some(value),
        lifetime: PhantomCovariantLifetime::new()
    }}
}

//> CONVERSIONS -> FROM OPTION NONNULL
const impl<Type> From<Option<NonNull<Type>>> for Pointer<'static, Type> {
    fn from(value: Option<NonNull<Type>>) -> Self {return Self {
        to: value,
        lifetime: PhantomCovariantLifetime::new()
    }}
}