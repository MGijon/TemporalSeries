use temporalseries::errors::TemporalSeriesError;
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_linear_series__when_compute_excess_kurtosis__then_returns_correct_value() {
    // Given
    // [1,2,3,4,5]: m2=2.0, m4=6.8 => 6.8/4.0 - 3 = -1.3
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5],
        ColumnarBackend::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]),
    )
    .unwrap();

    // When
    let result: f64 = sut.excess_kurtosis().unwrap();

    // Then
    assert!((result - (-1.3)).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_too_short_series__when_compute_excess_kurtosis__then_returns_empty_series_error() {
    // Given
    let sut: TS =
        TemporalSeries::new(vec![1, 2, 3], ColumnarBackend::new(vec![1.0, 2.0, 3.0])).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.excess_kurtosis();

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
