//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> FEATURES
#![feature(const_destruct)]
#![feature(const_option_ops)]
#![feature(const_trait_impl)]
#![feature(const_closures)]
#![feature(const_try)]
#![feature(const_default)]
#![feature(const_convert)]
#![feature(ptr_mask)]

//> HEAD -> MODULES
mod comparisons;
mod conversions;
#[cfg(test)]
mod tests;

//> HEAD -> CORE
use core::{
    ptr::{
        NonNull,
        null,
        null_mut
    },
    fmt::{
        Debug,
        Formatter,
        Result as Format
    },
    any::type_name,
    marker::Destruct
};


//^
//^ POINTER
//^

//> POINTER -> STRUCT
pub struct Pointer<Type> {
    to: Option<NonNull<Type>>
}

//> POINTER -> IMPLEMENTATION
impl<Type> Pointer<Type> {
    #[inline]
    pub const fn is_null(self) -> bool {return self.to.is_none()}
    #[inline]
    pub fn address(self) -> usize {return self.to.map(|pointer| pointer.addr().into()).unwrap_or_default()}
    #[inline]
    pub const fn add(self, count: usize) -> Pointer<Type> {return Self {
        to: self.to.map(const |pointer| unsafe {pointer.add(count)})
    }}
    #[inline]
    pub const fn sub(self, count: usize) -> Pointer<Type> {return Self {
        to: self.to.map(const |pointer| unsafe {pointer.sub(count)})
    }}
    #[inline]
    pub const unsafe fn read(self) -> Option<Type> {return Some(unsafe {self.to?.read()})}
    #[inline]
    pub const fn as_ptr(self) -> *const Type {return match self.to {
        None => null(),
        Some(pointer) => pointer.as_ptr()
    }}
    #[inline]
    pub const fn as_ptr_mut(self) -> *mut Type {return match self.to {
        None => null_mut(),
        Some(pointer) => pointer.as_ptr()
    }}
    #[inline]
    pub const fn get<'lifetime>(self) -> Option<&'lifetime Type> {return self.to.map(const |pointer| unsafe {pointer.as_ref()})}
    #[inline]
    pub const fn get_mut<'lifetime>(self) -> Option<&'lifetime mut Type> {return self.to.map(const |mut pointer| unsafe {pointer.as_mut()})}
    #[inline]
    pub const fn take(self) -> Option<NonNull<Type>> {return self.to}
    #[inline]
    pub fn is_aligned(self) -> bool {return self.address() % align_of::<Type>() == 0}
    #[inline]
    pub const fn cast<Other>(self) -> Pointer<Other> {return Pointer {
        to: self.to.map(const |pointer| pointer.cast())
    }}
    #[inline]
    pub fn mask(self, mask: usize) -> Pointer<Type> {Pointer::from(self.to.map(|pointer| pointer.as_ptr()).unwrap_or(null_mut()).mask(mask))}
}

//> POINTER -> IMPLEMENTATION WITH DESTRUCT
const impl<Type: [const] Destruct> Pointer<Type> {
    #[inline]
    pub unsafe fn write(self, value: Type) -> () {if let Some(pointer) = self.to {unsafe {pointer.write(value)}}}
    #[inline]
    pub unsafe fn replace(self, with: Type) -> Option<Type> {return Some(unsafe {self.to?.replace(with)})}
}

//> POINTER -> CLONE
impl<Type> Clone for Pointer<Type> {
    fn clone(&self) -> Self {return Self {
        to: self.to.clone()
    }}
}

//> POINTER -> COPY
impl<Type> Copy for Pointer<Type> {}

//> POINTER -> DEBUG
impl<Type: Debug> Debug for Pointer<Type> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {write!(formatter, "*{}", type_name::<Type>())}
}

//> POINTER -> DEFAULT
const impl<Type> Default for Pointer<Type> {
    fn default() -> Self {return Self {
        to: None
    }}
}

//> POINTER -> SEND
unsafe impl<Type> Send for Pointer<Type> {}

//> POINTER -> SYNC
unsafe impl<Type> Sync for Pointer<Type> {}