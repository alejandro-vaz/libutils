//^
//^ HEAD
//^

//> HEAD -> API
use libutils::issue::Issue;


//> HEAD -> CREATE
#[test]
fn create() -> () {
    let _issue = Issue {
        name: "die",
        description: Some("please".to_string())
    };
}