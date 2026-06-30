//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;

//> HEAD -> TEST
use test::Bencher;

//> HEAD -> CORE
use core::hint::black_box;


//^
//^ BENCHES
//^

//> BENCHES -> PUSHPOP
#[bench]
fn pushpop(bencher: &mut Bencher) -> () {
    let mut array = Array::<u8, 1>::new();
    bencher.iter(|| {
        for _ in 0..2usize.pow(2u32.pow(2u32.pow(2))) {
            black_box(array.push(black_box(0)));
            let x = black_box(array.pop());
            black_box(x);
        }
    });
}