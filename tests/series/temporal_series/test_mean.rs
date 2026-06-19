use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_zeros__when_compute_mean__then_returns_zero() {
    // Given
    let sut: TS =
        TemporalSeries::new(vec![1, 2, 3], ColumnarBackend::new(vec![0.0, 0.0, 0.0])).unwrap();

    // When
    let result: f64 = sut.mean();

    // Then
    assert_eq!(result, 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_linear_series__when_compute_mean__then_returns_arithmetic_mean() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4],
        ColumnarBackend::new(vec![1.0, 2.0, 3.0, 4.0]),
    )
    .unwrap();

    // When
    let result: f64 = sut.mean();

    // Then
    assert!((result - 2.5).abs() < 1e-9);
}
