//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    LAYOUT,
    OUTPUT
};

//> HEAD -> STD
use std::io::{
    stdout,
    Write
};

//> HEAD -> SYSTEMIO
use systemio::Update as UpdateTrait;

//> HEAD -> BYTEDIFF
use bytediff::Diff;


//^
//^ UPDATE
//^

//> UPDATE -> STRUCT
pub struct Update;

//> UPDATE -> TRAIT
impl UpdateTrait for Update {
    fn sync(self) -> () {
        let content = LAYOUT.get(|layout| {
            layout.iter().map(ToString::to_string).collect::<Vec<String>>()
        }).join("\n\n");
        OUTPUT.get(|output| {
            let mut lock = stdout().lock();
            lock.write(<Diff as Into<Vec<u8>>>::into(Diff::new(
                output.as_bytes(), 
                content.as_bytes()
            )).as_ref()).unwrap();
            lock.flush().unwrap();
            *output = content;
        });
    }
}