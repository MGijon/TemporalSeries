use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_rolling_std__then_returns_zeros() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3, 4], vec![5.0, 5.0, 5.0, 5.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.rolling_standard_deviation(2);

    // Then
    let std: TimeSeries = result.unwrap();
    assert!(std.values[0].is_nan());
    assert!(std.values[1] == 0.0);
    assert!(std.values[2] == 0.0);
    assert!(std.values[3] == 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_linear_series_window_3__when_compute_rolling_std__then_computes_correctly() {
    // Given
    // [1,2,3,4,5], window=3
    // Each window [1,2,3], [2,3,4], [3,4,5] has mean 2,3,4 and sample std = 1.0
    let sut: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.rolling_standard_deviation(3);

    // Then
    let std: TimeSeries = result.unwrap();
    assert!(std.values[0].is_nan());
    assert!(std.values[1].is_nan());
    assert!((std.values[2] - 1.0).abs() < 1e-9);
    assert!((std.values[3] - 1.0).abs() < 1e-9);
    assert!((std.values[4] - 1.0).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_invalid_window__when_compute_rolling_std__then_returns_error() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![1, 2], vec![1.0, 2.0]).unwrap();

    // When
    let result_too_large: Result<TimeSeries, TemporalSeriesError> =
        sut.rolling_standard_deviation(5);
    let result_window_1: Result<TimeSeries, TemporalSeriesError> =
        sut.rolling_standard_deviation(1);

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
