use std::time::Duration;
use chrono::NaiveDate;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use calendar_puzzle_solver::find_all_solutions;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "calendar puzzle",
        |b| b.iter(|| find_all_solutions(black_box(NaiveDate::from_ymd_opt(2023, 03, 21).unwrap())))
    );
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(60));
    targets = criterion_benchmark
);
criterion_main!(benches);
