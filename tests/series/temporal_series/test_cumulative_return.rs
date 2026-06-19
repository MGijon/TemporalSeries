use temporalseries::errors::TemporalSeriesError;
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_growing_series__when_compute_cumulative_return__then_computes_correctly() {
    // Given
    // (121 - 100) / 100 = 0.21
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3],
        ColumnarBackend::new(vec![100.0, 110.0, 121.0]),
    )
    .unwrap();

    // When
    let result: f64 = sut.cumulative_return().unwrap();

    // Then
    assert!((result - 0.21).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_cumulative_return__then_returns_zero() {
    // Given
    let sut: TS =
        TemporalSeries::new(vec![1, 2, 3], ColumnarBackend::new(vec![50.0, 50.0, 50.0])).unwrap();

    // When
    let result: f64 = sut.cumulative_return().unwrap();

    // Then
    assert_eq!(result, 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_empty_series__when_compute_cumulative_return__then_returns_empty_series_error() {
    // Given
    let sut: TS = TemporalSeries::new(vec![], ColumnarBackend::new(vec![])).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.cumulative_return();

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
