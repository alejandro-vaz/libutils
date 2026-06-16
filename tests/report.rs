//^
//^ HEAD
//^

//> HEAD -> API
use libutils::report::{
    Report,
    Act
};


//^
//^ TESTS
//^

//> TEST -> STRING
#[test]
fn string() -> () {
    let mut report = Report::default();
    report.warn("hello");
    assert_eq!(report.fail::<()>().result(), None);
}

//> TEST -> HIERARCHY
#[test]
fn hierarchy() -> Act<(), &'static str> {
    let mut superior = Report::default();
    superior.warn("hello");
    let inferior = Report::default().succeed(0);
    let number = superior.attach(inferior);
    assert_eq!(Some(0), number);
    superior.succeed(())
}