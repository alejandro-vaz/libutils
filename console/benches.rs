//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Argument;

//> HEAD -> ALLOC
use alloc::{
    string::ToString,
    vec::Vec
};

//> HEAD -> TEST
use test::Bencher;

//> HEAD -> CORE
use core::hint::black_box;


//^
//^ BENCHES
//^

//> BENCHES -> PARSING
#[bench]
fn parsing(bencher: &mut Bencher) -> () {
    let arguments = ["myexec.exect", "rm", "-rf", "--please", "--opt=true", "&"];
    bencher.iter(|| {
        for _ in 0..2usize.pow(2u32.pow(2)) {
            black_box(arguments.into_iter().map(ToString::to_string).map(Into::into).collect::<Vec<Argument>>());
        }
    });
}