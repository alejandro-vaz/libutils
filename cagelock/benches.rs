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

//> BENCHES -> ACCESS
#[bench]
fn access(bencher: &mut Bencher) -> () {
    let cage = Cage::new(3);
    bencher.iter(|| {
        for _ in 0..2usize.pow(2u32.pow(2u32.pow(2))) {
            let x = cage.read(|x| *x);
            black_box(x);
        }
    });
}