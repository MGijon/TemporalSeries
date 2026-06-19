use temporalseries::errors::TemporalSeriesError;
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_trending_series__when_dickey_fuller_test__then_returns_false() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        ColumnarBackend::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
    )
    .unwrap();

    // When
    let result: bool = sut.stationary_dickey_fuller_test(0.05).unwrap();

    // Then
    assert!(!result);
}

#[test]
#[allow(non_snake_case)]
fn test__given_alternating_series__when_dickey_fuller_test__then_returns_true() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        ColumnarBackend::new(vec![1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0]),
    )
    .unwrap();

    // When
    let result: bool = sut.stationary_dickey_fuller_test(0.05).unwrap();

    // Then
    assert!(result);
}

#[test]
#[allow(non_snake_case)]
fn test__given_short_series__when_dickey_fuller_test__then_returns_empty_series_error() {
    // Given
    let sut: TS = TemporalSeries::new(vec![1, 2], ColumnarBackend::new(vec![1.0, 2.0])).unwrap();

    // When
    let result: Result<bool, TemporalSeriesError> = sut.stationary_dickey_fuller_test(0.05);

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
