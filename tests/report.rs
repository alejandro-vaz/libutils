//^
//^ HEAD
//^

//> HEAD -> API
use libutils::report::{
    Report,
    Act,
    Note
};


//^
//^ TESTS
//^

//> TEST -> STRING
#[test]
fn string() -> () {
    let report = Report::new("test string");
    let act = report.fail::<()>("hello");
    assert_eq!(act.result, None);
}

//> TEST -> HIERARCHY
#[test]
fn hierarchy() -> Act<(), &'static str> {
    let mut report = Report::new("superior");
    let another = || {
        let inferior = Report::new("inferior");
        if true {
            inferior.succeed(5)
        } else {
            inferior.fail("failure")
        }
    };
    let _ = another().attach(&mut report)?;
    report.succeed(())
}

//> TEST -> TAKE
#[test]
fn take() -> Act<(), &'static str> {
    let mut report = Report::new("take");
    let _ = Some(2).note(&mut report, "failed")?;
    return report.succeed(());
}