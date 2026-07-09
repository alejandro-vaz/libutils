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
            array.push(black_box(0));
            let x = black_box(array.pop());
            black_box(x);
        }
    });
}

//> BENCHES -> INSERTREMOVE
#[bench]
fn insertremove(bencher: &mut Bencher) -> () {
    let mut array = Array::<u8, 5>::from([1, 2, 3]);
    bencher.iter(|| {
        for _ in 0..2usize.pow(2u32.pow(2u32.pow(2))) {
            array.insert(black_box(0), black_box(0));
            let x = array.remove(black_box(0));
            black_box(x);
        }
    });
}

//> BENCHES -> EXTENDCLEAR
#[bench]
fn extendclear(bencher: &mut Bencher) -> () {
    let mut array = Array::<u8, 10>::new();
    let source = [1, 2, 3, 4, 5];
    bencher.iter(|| {
        for _ in 0..2usize.pow(2u32.pow(2u32.pow(2))) {
            array.extend(black_box(source));
            array.clear();
        }
    });
}