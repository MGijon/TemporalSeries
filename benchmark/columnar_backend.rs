use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use temporalseries::storage::{ColumnarBackend, StorageBackend};

fn bench_columnar_new(c: &mut Criterion) {
    c.bench_function("ColumnarBackend::new 1000", |b| {
        b.iter(|| {
            let values: Vec<f64> = (0..1000).map(|i| i as f64).collect();
            ColumnarBackend::new(black_box(values))
        })
    });
}

fn bench_columnar_iter_sum(c: &mut Criterion) {
    let backend = ColumnarBackend::new((0..1000).map(|i| i as f64).collect::<Vec<f64>>());

    c.bench_function("ColumnarBackend iter sum 1000", |b| {
        b.iter(|| black_box(&backend).iter().copied().sum::<f64>())
    });
}

fn bench_columnar_get(c: &mut Criterion) {
    let backend = ColumnarBackend::new((0..1000).map(|i| i as f64).collect::<Vec<f64>>());

    c.bench_function("ColumnarBackend get 1000", |b| {
        b.iter(|| {
            for i in 0..1000 {
                black_box(black_box(&backend).get(i));
            }
        })
    });
}

criterion_group!(
    benches,
    bench_columnar_new,
    bench_columnar_iter_sum,
    bench_columnar_get
);
criterion_main!(benches);
