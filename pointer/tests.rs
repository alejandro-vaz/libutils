//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Pointer;


//^
//^ TESTS
//^

//> TESTS -> NEW
#[test]
fn new() -> () {
    let mut mytype = 5;
    let pointer = Pointer::from(&mut mytype);
    let other = Pointer::<()>::default();
    assert!(!pointer.is_null());
    assert!(other.is_null());
}

//> TESTS -> ADDRESS
#[test]
fn address() -> () {
    let mut mytype = 5;
    let mutable = &mut mytype;
    let raw = mutable as *mut i32;
    let pointer = Pointer::from(mutable);
    assert_eq!(pointer.address(), raw.addr());
}

//> TESTS -> OPERATIONS
#[test]
fn operations() -> () {
    let mut mytype = 5;
    let pointer = Pointer::from(&mut mytype);
    let operated = pointer.add(3).sub(3);
    assert_eq!(pointer.address(), operated.address());
}

//> TESTS -> RW
#[test]
fn rw() -> () {
    let mut mytype = 5;
    let pointer = Pointer::from(&mut mytype);
    let read = unsafe {pointer.read()};
    assert_eq!(read, Some(5));
    unsafe {pointer.write(7)};
    assert_eq!(unsafe {pointer.read()}, Some(7));
}

//> TESTS -> REPLACE
#[test]
fn replace() -> () {
    let mut mytype = 5;
    let pointer = Pointer::from(&mut mytype);
    let old = unsafe {pointer.replace(6)};
    assert_eq!(old, Some(5));
    assert_eq!(unsafe {pointer.read()}, Some(6));
}

//> TESTS -> ASPTR
#[test]
fn asptr() -> () {
    let mut mytype = 5;
    let pointer = Pointer::from(&mut mytype);
    assert_eq!(pointer.as_ptr_mut(), &raw mut mytype);
}