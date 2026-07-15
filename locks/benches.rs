//^
//^ HEAD
//^

//> HEAD -> CAGELOCK
use locks::Mutex;

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
criterion_group!(locks, benches);
criterion_main!(locks);

//> BENCHES -> RUN
fn benches(criterion: &mut Criterion) -> () {
    let mut group = criterion.benchmark_group("locks");
    const ITERATIONS: usize = 10000;
    group.throughput(Throughput::Elements(ITERATIONS as u64));
    let cage = Mutex::new(3);
    group.bench_function("access", |bencher| bencher.iter(|| {for _ in 0..ITERATIONS {
        let x = cage.with(|x| *x);
        black_box(x);
    }}));
}
