//^
//^ HEAD
//^

//> HEAD -> API
use libutils::report::Report;

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
const ITERATIONS: u64 = 2u64.pow(12);

//> BENCHES -> BENCH
fn bench(criterion: &mut Criterion) -> () {
    let mut group = criterion.benchmark_group("report");
    group.throughput(Throughput::Elements(ITERATIONS));
    group.bench_function("attach", |bencher| bencher.iter(attach));
    group.finish();
}

//> BENCHES -> ATTACH
fn attach() -> () {
    let mut superior = Report::<&'static str>::new("superior");
    for _ in 0..ITERATIONS {
        let _ = black_box(Report::new("inferior").finish(black_box(Ok(()))).attach(black_box(&mut superior)));
    }
}

//> BENCHES -> SETUP
criterion_group!(benches, bench);
criterion_main!(benches);