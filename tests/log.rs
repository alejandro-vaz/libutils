//^
//^ HEAD
//^

//> HEAD -> API
use libutils::log::Log;


//^
//^ LOG
//^

//> LOG -> BASIC
#[test]
fn basic() -> () {
    let mut log = Log::from(&[1, 2, 3]);
    assert_eq!(log.len(), 3);
    log.push(&4);
    assert_eq!(log.len(), 4);
}

//> LOG -> EXTENDITER
#[test]
fn extenditer() -> () {
    let mut log = Log::default();
    log.extend([1, 2, 3]);
    for (index, value) in log.into_iter().enumerate() {
        assert_eq!(index + 1, value)
    }
}

//> LOG -> REFERENCES
#[test]
fn references() -> () {
    let array = [1, 2, 3];
    let log = Log::from(array);
    assert_eq!(log.as_ref(), &array);
}