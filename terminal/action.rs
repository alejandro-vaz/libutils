//^
//^ HEAD
//^

//> HEAD -> CONSOLE
use libutils_console::Handle;

//> HEAD -> SUPER
use super::TERMINAL;

//> HEAD -> DIFF
use libutils_diff::Diff;

//> HEAD -> STD
use std::io::{
    stdout,
    Write
};


//^
//^ ACTION
//^

//> ACTION -> STRUCT
pub struct Action;

//> ACTION -> HANDLE
impl Handle for Action {
    #[inline]
    fn sync(self) -> () {
        let mut content = TERMINAL.layout.read(|layout| layout.iter().map(ToString::to_string).collect::<Vec<String>>()).join("\n\n");
        content.push('\n');
        TERMINAL.output.write(|output| {
            let mut lock = stdout().lock();
            lock.write(<Diff as Into<Vec<u8>>>::into(Diff::new(
                output.as_bytes(), 
                content.as_bytes()
            )).as_ref()).unwrap();
            lock.flush().unwrap();
            *output = content;
        });
    }
    #[inline]
    fn ignore(self) -> () {}
}