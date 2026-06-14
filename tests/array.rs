//^
//^ HEAD
//^

//> HEAD -> API
use libutils::array::Array;


//^
//^ TESTS
//^

//> TESTS -> BASIC
#[test]
fn pushpop() -> () {
    let mut new = Array::<u8, 5>::new();
    new.push(0);
    new.push(1);
    new.push(2);
    new.push(3);
    new.push(4);
    new.pop();
    new.push(5);
    assert_eq!(new.as_ref(), &[0, 1, 2, 3, 5]);
}