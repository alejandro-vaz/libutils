//^
//^ HEAD
//^

//> HEAD -> API
use libutils::report::Report;


//^
//^ TESTS
//^

//> TEST -> STRING
#[test]
fn string() -> () {
    let mut report: Report<&'static str> = Report::default();
    report.warn("hello");
    assert_eq!(report.conclude(0).value, Some(0));
}

//> TEST -> HIERARCHY
#[test]
fn hierarchy() -> () {
    let mut superior = Report::default();
    superior.warn("hello");
    let inferior = Report::default().conclude(0);
    let number = superior.attach(inferior);
    assert_eq!(Some(0), number);
    superior.conclude(());
}