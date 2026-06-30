//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Cage;

//> HEAD -> TEST
use test::Bencher;

//> HEAD -> CORE
use core::hint::black_box;



//^
//^ BENCHES
//^

//> BENCHES -> READ
#[bench]
fn read(bencher: &mut Bencher) -> () {
    let cage = Cage::new(3);
    bencher.iter(|| {
        for _ in 0..2usize.pow(2u32.pow(2u32.pow(2))) {
            let x = cage.read();
            let _u = black_box(x);
        }
    });
}