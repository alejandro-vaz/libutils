//^
//^ HEAD
//^

//> HEAD -> CORE
use core::hint::black_box;

//> HEAD -> SUPER
use super::reduce;

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
        let x = reduce(black_box(&ebnf));
        black_box(x);
    });
}