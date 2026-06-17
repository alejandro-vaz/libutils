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
    let report = Report::default();
    let act = report.fail::<()>("hello");
    assert_eq!(act.result, None);
}

//> TEST -> HIERARCHY
#[test]
fn hierarchy() -> Act<(), &'static str> {
    let mut report = Report::default();
    let another = || {
        let inferior = Report::default();
        if true {
            inferior.succeed(5)
        } else {
            inferior.fail("failure")
        }
    };
    let _ = report.attach(another())?;
    report.succeed(())
}