//^
//^ HEAD
//^

//> HEAD -> API
use libutils::terminal::TERMINAL;


//^ 
//^ TESTS
//^ 

//> TESTS -> CLI
#[test]
fn cli() -> () {
    let _arguments = TERMINAL.read().arguments();
    let _vars = TERMINAL.read().environment();
}