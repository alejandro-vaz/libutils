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
    let report = Report::<_, "test string">::default();
    let act = report.finish::<()>(Err("hello"));
    assert_eq!(act.result, None);
}

//> TEST -> HIERARCHY
#[test]
fn hierarchy() -> Act<(), &'static str, "Main"> {
    let mut report = Report::default();
    let another = || {
        let inferior = Report::<_, "inferior">::default();
        if true {
            inferior.finish(Ok(()))
        } else {
            inferior.finish(Err("failure"))
        }
    };
    let _ = another().attach(&mut report)?;
    return report.finish(Ok(()));
}

//> TEST -> TAKE
#[test]
fn take() -> Act<(), &'static str, "Main"> {
    let mut report = Report::default();
    let _ = Some(2).note(&mut report, "failed")?;
    return report.finish(Ok(()));
}