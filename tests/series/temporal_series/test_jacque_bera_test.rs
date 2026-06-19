use temporalseries::errors::TemporalSeriesError;
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_near_normal_series__when_jacque_bera_test__then_returns_true() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5],
        ColumnarBackend::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]),
    )
    .unwrap();

    // When
    let result: bool = sut.jacque_bera_test(0.05).unwrap();

    // Then
    assert!(result);
}

#[test]
#[allow(non_snake_case)]
fn test__given_heavily_skewed_series__when_jacque_bera_test__then_returns_false() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        ColumnarBackend::new(vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 100.0]),
    )
    .unwrap();

    // When
    let result: bool = sut.jacque_bera_test(0.05).unwrap();

    // Then
    assert!(!result);
}

#[test]
#[allow(non_snake_case)]
fn test__given_too_short_series__when_jacque_bera_test__then_returns_empty_series_error() {
    // Given
    let sut: TS =
        TemporalSeries::new(vec![1, 2, 3], ColumnarBackend::new(vec![1.0, 2.0, 3.0])).unwrap();

    // When
    let result: Result<bool, TemporalSeriesError> = sut.jacque_bera_test(0.05);

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
