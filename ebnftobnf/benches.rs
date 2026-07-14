//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(default_field_values)]

//> HEAD -> CORE
use core::hint::black_box;

//> HEAD -> SUPER
use ebnftobnf::{
    reduce,
    Settings
};

//> HEAD -> CRITERION
use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
    Throughput
};


//^
//^ BENCHES
//^

//> BENCHES -> SETUP
criterion_group!(ebnftobnf, benches);
criterion_main!(ebnftobnf);

//> BENCHES -> RUN
fn benches(criterion: &mut Criterion) -> () {
    let mut group = criterion.benchmark_group("ebnftobnf");
    const ITERATIONS: usize = 10;
    group.throughput(Throughput::Bytes((ITERATIONS * "A: B* (C D+)?\n".len()) as u64));
    let ebnf = "A: B* (C D+)?\n".repeat(ITERATIONS);
    group.bench_function("reducing", |bencher| bencher.iter(|| {
        let x = reduce(black_box(&ebnf), black_box(Settings {
            keep_comments: false,
            keep_empty_lines: false,
            ..
        }));
        black_box(x);
    }));
}