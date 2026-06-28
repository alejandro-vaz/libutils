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
fn string() -> Act<()> {
    let main = Report::default();
    return main.with_default();
}

//> TEST -> HIERARCHY
#[test]
fn hierarchy() -> Act<()> {
    let report = Report::default();
    let another = |inferior: Report<"Inferior", _>| -> Act<()> {
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
fn take() -> Act<()> {
    let report = Report::default();
    report.warn("die");
    return report.with_default();
}