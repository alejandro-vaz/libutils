//^
//^ HEAD
//^

//> HEAD -> API
use libutils::report::Report;

//> HEAD -> CORE
use core::{
    hint::black_box,
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

//> BENCHES -> ITERATIONS
const ITERATIONS: u64 = 2u64.pow(15);

//> BENCHES -> BENCH
fn bench(criterion: &mut Criterion) -> () {
    let mut group = criterion.benchmark_group("report");
    group.sample_size(1000);
    group.confidence_level(0.975);
    group.significance_level(0.025);
    group.measurement_time(Duration::from_secs(10));
    group.warm_up_time(Duration::from_secs(5));
    group.throughput(Throughput::Elements(ITERATIONS));
    group.bench_function("attach", |bencher| bencher.iter(attach));
    group.finish();
}

//> BENCHES -> ATTACH
fn attach() -> () {
    let mut superior = Report::<&'static str, _>::default();
    for _ in 0..ITERATIONS {
        black_box(superior.attach(black_box(Report::default().conclude(black_box(())))));
    }
}

//> BENCHES -> SETUP
criterion_group!(benches, bench);
criterion_main!(benches);