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
    assert_eq!(report.conclude(0).result(), Some(0));
}