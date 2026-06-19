use temporalseries::errors::TemporalSeriesError;
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_symmetric_series__when_compute_skewness__then_returns_zero() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5],
        ColumnarBackend::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]),
    )
    .unwrap();

    // When
    let result: f64 = sut.skewness().unwrap();

    // Then
    assert!(result.abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_right_skewed_series__when_compute_skewness__then_returns_positive_value() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5],
        ColumnarBackend::new(vec![1.0, 1.0, 1.0, 1.0, 5.0]),
    )
    .unwrap();

    // When
    let result: f64 = sut.skewness().unwrap();

    // Then
    assert!(result > 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_too_short_series__when_compute_skewness__then_returns_empty_series_error() {
    // Given
    let sut: TS = TemporalSeries::new(vec![1, 2], ColumnarBackend::new(vec![1.0, 2.0])).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.skewness();

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
