//^
//^ HEAD
//^

//> HEAD -> CORE
use core::hint::black_box;

//> HEAD -> SUPER
use super::{
    reduce,
    Settings
};

//> HEAD -> TEST
use test::Bencher;


//^
//^ BENCHES
//^

//> BENCHES -> REDUCING
#[bench]
fn reducing(bencher: &mut Bencher) -> () {
    let ebnf = "A: B* (C D+)?\n".repeat(10);
    bencher.iter(|| {
        let x = reduce(black_box(&ebnf), black_box(Settings {
            keep_comments: false,
            keep_empty_lines: false,
            ..
        }));
        black_box(x);
    });
}