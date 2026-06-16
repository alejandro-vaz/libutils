//^
//^ HEAD
//^

//> HEAD -> API
use libutils::cage::Cage;

//> HEAD -> CORE
use core::{
    hint::black_box,
    ops::AddAssign,
};

//> HEAD -> CRITERION
use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
    Throughput
};


//^
//^ BENCHES
//^

//> BENCHES -> ITERATIONS
const ITERATIONS: u64 = 2u64.pow(15);

//> BENCHES -> BENCH
fn bench(criterion: &mut Criterion) -> () {
    let mut group = criterion.benchmark_group("cage");
    group.throughput(Throughput::Elements(ITERATIONS));
    group.bench_function("free", |bencher| bencher.iter(free));
    group.finish();
}

//> BENCHES -> FREE
fn free() -> () {
    let cage = Cage::new(0usize);
    for _ in 0..ITERATIONS {black_box(cage.free().add_assign(1))}
}

//> BENCHES -> SETUP
criterion_group!(benches, bench);
criterion_main!(benches);