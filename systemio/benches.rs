//^
//^ HEAD
//^

//> HEAD -> SYSTEMIO
use systemio::Argument;

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
criterion_group!(systemio, benches);
criterion_main!(systemio);

//> BENCHES -> RUN
fn benches(criterion: &mut Criterion) -> () {
    let mut group = criterion.benchmark_group("systemio");
    const ITERATIONS: usize = 100;
    let arguments = ["myexec.exect", "rm", "-rf", "--please", "--opt=true", "&"];
    group.throughput(Throughput::Elements((ITERATIONS * arguments.len()) as u64));
    group.bench_function("parsing", |bencher| bencher.iter(|| for _ in 0..ITERATIONS {
        black_box(arguments.into_iter().map(ToString::to_string).map(Into::into).collect::<Vec<Argument>>());
    }));
}