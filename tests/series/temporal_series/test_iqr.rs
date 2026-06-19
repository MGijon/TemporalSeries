use temporalseries::errors::TemporalSeriesError;
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_linear_series__when_compute_iqr__then_returns_correct_spread() {
    // Given
    // [1,2,3,4,5]: Q1=2.0, Q3=4.0, IQR=2.0
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5],
        ColumnarBackend::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]),
    )
    .unwrap();

    // When
    let result: f64 = sut.iqr().unwrap();

    // Then
    assert_eq!(result, 2.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_iqr__then_returns_zero() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4],
        ColumnarBackend::new(vec![7.0, 7.0, 7.0, 7.0]),
    )
    .unwrap();

    // When
    let result: f64 = sut.iqr().unwrap();

    // Then
    assert_eq!(result, 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_empty_series__when_compute_iqr__then_returns_empty_series_error() {
    // Given
    let sut: TS = TemporalSeries::new(vec![], ColumnarBackend::new(vec![])).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.iqr();

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
