use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use temporalseries::series::TimeSeries;

fn bench_pct_change(c: &mut Criterion) {
    let index: Vec<i64> = (0..1000).collect();
    let values: Vec<f64> = (0..1000).map(|i| 100.0 + i as f64).collect();
    let ts = TimeSeries::new(index, values).unwrap();

    c.bench_function("pct_change 1000", |b| {
        b.iter(|| black_box(&ts).pct_change().unwrap())
    });
}

fn bench_rolling_mean(c: &mut Criterion) {
    let index: Vec<i64> = (0..1000).collect();
    let values: Vec<f64> = (0..1000).map(|i| 100.0 + i as f64).collect();
    let ts = TimeSeries::new(index, values).unwrap();

    c.bench_function("rolling mean window=20 1000", |b| {
        b.iter(|| black_box(&ts).rolling(20).mean().unwrap())
    });
}

criterion_group!(benches, bench_pct_change, bench_rolling_mean);
criterion_main!(benches);