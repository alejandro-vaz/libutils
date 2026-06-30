//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Log;

//> HEAD -> TEST
use test::Bencher;

//> HEAD -> CORE
use core::hint::black_box;



//^
//^ BENCHES
//^

//> BENCHES -> PUSH
#[bench]
fn push(bencher: &mut Bencher) -> () {
    bencher.iter(|| {
        let mut log = Log::new();
        for _ in 0..2usize.pow(2u32.pow(2u32.pow(2))) {
            log.push(black_box(0));
        }
    });
}