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
const ITERATIONS: u64 = 2u64.pow(14);

//> BENCHES -> BENCH
fn bench(criterion: &mut Criterion) -> () {
    let mut group = criterion.benchmark_group("cage");
    group.throughput(Throughput::Elements(ITERATIONS));
    group.bench_function("read", |bencher| bencher.iter(read));
    group.bench_function("write", |bencher| bencher.iter(write));
    group.finish();
}

//> BENCHES -> READ
fn read() -> () {
    let cage = Cage::new("hello");
    for _ in 0..ITERATIONS {
        let ptr = black_box(cage.read().as_ptr());
        black_box(ptr);
    }
}

//> BENCHES -> WRITE
fn write() -> () {
    let cage = Cage::new(0usize);
    for _ in 0..ITERATIONS {black_box(cage.write().add_assign(1))}
}

//> BENCHES -> SETUP
criterion_group!(benches, bench);
criterion_main!(benches);