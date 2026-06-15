//^
//^ HEAD
//^

//> HEAD -> API
use libutils::array::Array;

//> HEAD -> CORE
use core::hint::black_box;

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
    let mut group = criterion.benchmark_group("array");
    group.throughput(Throughput::Elements(ITERATIONS));
    group.bench_function("push", |bencher| bencher.iter(push));
    group.bench_function("pushpop", |bencher| bencher.iter(pushpop));
    group.bench_function("insertremove", |bencher| bencher.iter(insertremove));
    group.bench_function("extend", |bencher| bencher.iter(extend));
    group.finish();
}

//> BENCHES -> PUSH
fn push() -> () {
    let mut array = Array::<u64, {ITERATIONS as usize}>::new();
    for index in 0..ITERATIONS {black_box(array.push(black_box(index)))}
}

//> BENCHES -> PUSHPOP
fn pushpop() -> () {
    let mut array = Array::<u64, 1>::new();
    for index in 0..ITERATIONS {
        black_box(array.push(black_box(index)));
        let removed = black_box(array.pop());
        black_box(removed);
    }
}

//> BENCHES -> INSERTREMOVE
fn insertremove() -> () {
    let mut array = Array::<u64, 4>::from([0, 0, 0]);
    for index in 0..ITERATIONS {
        black_box(array.insert(black_box(0), black_box(index)));
        let value = black_box(array.remove(black_box(1)));
        black_box(value);
    }
}

//> BENCHES -> EXTEND
fn extend() -> () {
    let mut array = Array::<u64, {ITERATIONS as usize * 2}>::new();
    for index in 0..ITERATIONS {black_box(array.extend(black_box([index, index])))}
}

//> BENCHES -> SETUP
criterion_group!(benches, bench);
criterion_main!(benches);