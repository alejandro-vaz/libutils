//^
//^ HEAD
//^

//> HEAD -> BYTEDIFF
use bytediff::Diff;

//> HEAD -> CORE
use core::hint::black_box;

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
criterion_group!(bytediff, benches);
criterion_main!(bytediff);

//> BENCHES -> RUN
fn benches(criterion: &mut Criterion) -> () {
    let mut group = criterion.benchmark_group("bytediff");
    const ITERATIONS: usize = 10000;
    group.throughput(Throughput::Bytes(ITERATIONS as u64));
    let void = [0].repeat(ITERATIONS);
    let mut big = [0].repeat(ITERATIONS);
    big.push(0);
    group.bench_function("new", |bencher| bencher.iter(|| {
        black_box(Diff::new(black_box(&void), black_box(&big)));
    }));
}