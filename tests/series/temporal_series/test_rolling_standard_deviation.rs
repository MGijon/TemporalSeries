use temporalseries::errors::TemporalSeriesError;
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_rolling_std__then_returns_zeros() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4],
        ColumnarBackend::new(vec![5.0, 5.0, 5.0, 5.0]),
    )
    .unwrap();

    // When
    let result = sut.rolling_standard_deviation(2).unwrap();
    let values: Vec<f64> = result.iter().copied().collect();

    // Then
    assert!(values[0].is_nan());
    assert_eq!(values[1], 0.0);
    assert_eq!(values[2], 0.0);
    assert_eq!(values[3], 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_linear_series_window_3__when_compute_rolling_std__then_computes_correctly() {
    // Given
    // Each window [1,2,3], [2,3,4], [3,4,5] has sample std = 1.0
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5],
        ColumnarBackend::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]),
    )
    .unwrap();

    // When
    let result = sut.rolling_standard_deviation(3).unwrap();
    let values: Vec<f64> = result.iter().copied().collect();

    // Then
    assert!(values[0].is_nan());
    assert!(values[1].is_nan());
    assert!((values[2] - 1.0).abs() < 1e-9);
    assert!((values[3] - 1.0).abs() < 1e-9);
    assert!((values[4] - 1.0).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_invalid_window__when_compute_rolling_std__then_returns_invalid_window_error() {
    // Given
    let sut: TS = TemporalSeries::new(vec![1, 2], ColumnarBackend::new(vec![1.0, 2.0])).unwrap();

    // When
    let result_too_large: Result<_, TemporalSeriesError> = sut.rolling_standard_deviation(5);
    let result_window_1: Result<_, TemporalSeriesError> = sut.rolling_standard_deviation(1);

    // Then
    assert!(matches!(
        result_too_large,
        Err(TemporalSeriesError::InvalidWindow { .. })
    ));
    assert!(matches!(
        result_window_1,
        Err(TemporalSeriesError::InvalidWindow { .. })
    ));
}
