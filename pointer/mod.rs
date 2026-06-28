//^
//^ HEAD
//^

//> HEAD -> MODULES
mod comparisons;
mod conversions;

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
    marker::PhantomCovariantLifetime
};


//^
//^ POINTER
//^

//> POINTER -> STRUCT
pub struct Pointer<'valid, Type> {
    to: Option<NonNull<Type>>,
    lifetime: PhantomCovariantLifetime<'valid>
}

//> POINTER -> IMPLEMENTATION
impl<'valid, Type> Pointer<'valid, Type> {
    #[inline]
    pub const fn is_null(self) -> bool {return self.to.is_none()}
    #[inline]
    pub fn address(self) -> usize {return self.to.map(|pointer| pointer.addr().into()).unwrap_or_default()}
    #[inline]
    pub const fn add(self, count: usize) -> Pointer<'valid, Type> {return Self {
        to: self.to.map(const |pointer| unsafe {pointer.add(count)}),
        lifetime: PhantomCovariantLifetime::new()
    }}
    #[inline]
    pub const fn sub(self, count: usize) -> Pointer<'valid, Type> {return Self {
        to: self.to.map(const |pointer| unsafe {pointer.sub(count)}),
        lifetime: PhantomCovariantLifetime::new()
    }}
    #[inline]
    pub const fn of(value: &'valid mut Type) -> Self {return Self {
        to: NonNull::new(value as *mut Type),
        lifetime: PhantomCovariantLifetime::new()
    }}
    #[inline]
    pub const unsafe fn read(self) -> Option<Type> {return Some(unsafe {self.to?.read()})}
    #[inline]
    pub unsafe fn write(self, value: Type) -> () {if let Some(pointer) = self.to {unsafe {pointer.write(value)}}}
    #[inline]
    pub unsafe fn replace(self, with: Type) -> Option<Type> {return Some(unsafe {self.to?.replace(with)})}
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
    pub const fn cast<Other>(self) -> Pointer<'valid, Other> {return Pointer {
        to: self.to.map(const |pointer| pointer.cast()),
        lifetime: PhantomCovariantLifetime::new()
    }}
}

//> POINTER -> CLONE
impl<'valid, Type> Clone for Pointer<'valid, Type> {
    fn clone(&self) -> Self {return Self {
        to: self.to.clone(),
        lifetime: self.lifetime
    }}
}

//> POINTER -> COPY
impl<'valid, Type> Copy for Pointer<'valid, Type> {}

//> POINTER -> DEBUG
impl<'valid, Type: Debug> Debug for Pointer<'valid, Type> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {write!(formatter, "Pointer({})", type_name::<Type>())}
}

//> POINTER -> DEFAULT
impl<Type> const Default for Pointer<'static, Type> {
    fn default() -> Self {return Self {
        to: None,
        lifetime: PhantomCovariantLifetime::new()
    }}
}

//> POINTER -> SEND
unsafe impl<'valid, Type> Send for Pointer<'valid, Type> {}

//> POINTER -> SYNC
unsafe impl<'valid, Type> Sync for Pointer<'valid, Type> {}