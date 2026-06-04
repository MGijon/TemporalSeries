use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use temporalseries::panel::Panel;

const N_TIMESTAMPS: usize = 1000;
const N_SYMBOLS: usize = 10;

fn make_panel() -> Panel {
    let index: Vec<i64> = (0..N_TIMESTAMPS as i64).collect();
    let symbols: Vec<String> = (0..N_SYMBOLS).map(|i| format!("SYM{i}")).collect();
    let values: Vec<Vec<f64>> = (0..N_SYMBOLS)
        .map(|s| {
            (0..N_TIMESTAMPS)
                .map(|t| (s * N_TIMESTAMPS + t) as f64)
                .collect()
        })
        .collect();
    Panel::new(index, symbols, values).unwrap()
}

fn bench_panel_new(c: &mut Criterion) {
    c.bench_function("Panel::new 1000x10", |b| {
        b.iter(|| {
            let index: Vec<i64> = (0..N_TIMESTAMPS as i64).collect();
            let symbols: Vec<String> = (0..N_SYMBOLS).map(|i| format!("SYM{i}")).collect();
            let values: Vec<Vec<f64>> = (0..N_SYMBOLS)
                .map(|s| {
                    (0..N_TIMESTAMPS)
                        .map(|t| (s * N_TIMESTAMPS + t) as f64)
                        .collect()
                })
                .collect();
            Panel::new(black_box(index), black_box(symbols), black_box(values)).unwrap()
        })
    });
}

fn bench_panel_get_series(c: &mut Criterion) {
    let panel = make_panel();

    c.bench_function("Panel::get_series last symbol 1000x10", |b| {
        // Last symbol is the worst case for the linear scan.
        b.iter(|| black_box(&panel).get_series("SYM9").unwrap())
    });
}

fn bench_panel_shape(c: &mut Criterion) {
    let panel = make_panel();

    c.bench_function("Panel::shape 1000x10", |b| {
        b.iter(|| black_box(&panel).shape())
    });
}

criterion_group!(
    benches,
    bench_panel_new,
    bench_panel_get_series,
    bench_panel_shape
);
criterion_main!(benches);
