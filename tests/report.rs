//^
//^ HEAD
//^

//> HEAD -> API
use libutils::report::{
    Report,
    Act,
    Set
};


//^
//^ TESTS
//^

//> TEST -> STRING
#[test]
fn string() -> Act<(), "Main"> {
    let main = Report::default();
    return main.with_default();
}

//> TEST -> HIERARCHY
#[test]
fn hierarchy() -> Act<(), "Main"> {
    let report = Report::default();
    let another = |inferior: Report<Set<'_, "Inferior">>| -> Act<(), "Inferior"> {
        if true {
            inferior.with_default()
        } else {
            inferior.fail("error")
        }
    };
    let _ = another(report.sub())?;
    return report.with_default();
}

//> TEST -> TAKE
#[test]
fn take() -> Act<(), "Main"> {
    let report = Report::default();
    let _ = Some(2)?;
    return report.with_default();
}