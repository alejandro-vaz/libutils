//^
//^ HEAD
//^

//> HEAD -> API
use libutils::cage::Cage;

//> HEAD -> STD
use std::{
    hint::black_box, 
    ops::AddAssign, 
    time::Duration
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

//> BENCHES -> DATA
const ITERATIONS: u64 = 2u64.pow(18);

//> BENCHES -> BENCH
fn bench(criterion: &mut Criterion) -> () {
    let mut group = criterion.benchmark_group("cage");
    group.sample_size(1000);
    group.confidence_level(0.975);
    group.significance_level(0.025);
    group.measurement_time(Duration::from_secs(10));
    group.warm_up_time(Duration::from_secs(5));
    group.throughput(Throughput::Elements(ITERATIONS));
    group.bench_function("free", |bencher| bencher.iter(free));
    group.finish();
}

//> BENCHES -> FREE
fn free() -> () {
    let cage = Cage::new(0usize);
    for _ in 0..ITERATIONS {black_box(cage.free(|x| x.add_assign(1)))}
}

//> BENCHES -> SETUP
criterion_group!(benches, bench);
criterion_main!(benches);