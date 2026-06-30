//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    Report,
    Stay,
    Add
};


//^
//^ TESTS
//^

//> TESTS -> USAGE
#[test]
fn usage() -> () {
    let mut report = Report::new("Main");
    let passing = |report: Report<Stay<'_>>| {
        report.critical("hello");
    };
    passing(report.to());
    let upgrading = |report: Report<Add<'_, "Category">>| {
        let chain = report.warn("hello");
    };
    upgrading(report.to());
    report.warn("outside");
}