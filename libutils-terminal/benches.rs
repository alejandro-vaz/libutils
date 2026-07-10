//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Terminal;

//> HEAD -> CONSOLE
use libutils_console::{
    Console, Descriptor, Update
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
            black_box(Terminal::print(black_box("hello!"))).sync();
        }
        Terminal::clear().sync();
    });
}

//> BENCHES -> OPEN
#[bench]
fn open(bencher: &mut Bencher) -> () {
    bencher.iter(|| {
        for _ in 0..100 {
            Terminal::open("README.md").unwrap().close();
        }
    });
}