use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use temporalseries::storage::{RowBackend, RowRecord, StorageBackend};

fn bench_row_new(c: &mut Criterion) {
    c.bench_function("RowBackend::new 1000", |b| {
        b.iter(|| {
            let rows: Vec<RowRecord<f64>> = (0..1000)
                .map(|i| RowRecord {
                    timestamp: i,
                    value: i as f64,
                })
                .collect();
            RowBackend::new(black_box(rows))
        })
    });
}

fn bench_row_iter_sum(c: &mut Criterion) {
    let rows: Vec<RowRecord<f64>> = (0..1000)
        .map(|i| RowRecord {
            timestamp: i,
            value: i as f64,
        })
        .collect();
    let backend = RowBackend::new(rows);

    c.bench_function("RowBackend iter sum 1000", |b| {
        b.iter(|| black_box(&backend).iter().copied().sum::<f64>())
    });
}

fn bench_row_get(c: &mut Criterion) {
    let rows: Vec<RowRecord<f64>> = (0..1000)
        .map(|i| RowRecord {
            timestamp: i,
            value: i as f64,
        })
        .collect();
    let backend = RowBackend::new(rows);

    c.bench_function("RowBackend get 1000", |b| {
        b.iter(|| {
            for i in 0..1000 {
                black_box(black_box(&backend).get(i));
            }
        })
    });
}

criterion_group!(benches, bench_row_new, bench_row_iter_sum, bench_row_get);
criterion_main!(benches);
