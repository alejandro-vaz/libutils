//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Diff;

//> HEAD -> TEST
use test::Bencher;

//> HEAD -> CORE
use core::hint::black_box;


//^
//^ BENCHES
//^

//> BENCHES -> NEW
#[bench]
fn new(bencher: &mut Bencher) -> () {
    const ITERATIONS: usize = 2usize.pow(2u32.pow(2u32.pow(2)));
    let void = [0].repeat(ITERATIONS);
    let mut big = [0].repeat(ITERATIONS);
    big.push(0);
    bencher.iter(|| {
        let x = Diff::new(black_box(&void), black_box(&big));
        black_box(x);
    });
}