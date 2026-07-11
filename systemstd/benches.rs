//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::System;

//> HEAD -> SYSTEMIO
use systemio::{
    SystemIO, 
    Descriptor, 
    Update
};

//> HEAD -> CORE
use core::hint::black_box;

//> HEAD -> TEST
use test::Bencher;


//^
//^ BENCHES
//^

//> BENCHES -> PRINT
#[bench]
fn print(bencher: &mut Bencher) -> () {
    bencher.iter(|| {
        for _ in 0..100 {
            black_box(System::print(black_box("hello!"))).sync();
        }
        System::clear().sync();
    });
}

//> BENCHES -> OPEN
#[bench]
fn open(bencher: &mut Bencher) -> () {
    bencher.iter(|| {
        for _ in 0..100 {
            System::open("README.md").unwrap().close();
        }
    });
}