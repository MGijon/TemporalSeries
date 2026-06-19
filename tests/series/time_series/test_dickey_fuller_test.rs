use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_linear_trend__when_compute_dickey_fuller_test__then_returns_non_stationary() {
    // Given
    // Pure upward trend is non-stationary — DF statistic >> -1.95, H0 not rejected
    let sut: TimeSeries = TimeSeries::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
    )
    .unwrap();

    // When
    let result: Result<bool, TemporalSeriesError> = sut.stationary_dickey_fuller_test(0.05);

    // Then
    assert!(!result.unwrap());
}

#[test]
#[allow(non_snake_case)]
fn test__given_mean_reverting_series__when_compute_dickey_fuller_test__then_returns_stationary() {
    // Given
    // Alternating ±1 around zero — strongly mean-reverting, DF statistic -> -inf
    let sut: TimeSeries = TimeSeries::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        vec![1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0],
    )
    .unwrap();

    // When
    let result: Result<bool, TemporalSeriesError> = sut.stationary_dickey_fuller_test(0.05);

    // Then
    assert!(result.unwrap());
}

#[test]
#[allow(non_snake_case)]
fn test__given_short_series__when_compute_dickey_fuller_test__then_returns_error() {
    // Given
    // Requires at least 3 elements
    let sut: TimeSeries = TimeSeries::new(vec![1, 2], vec![1.0, 2.0]).unwrap();

    // When
    let result: Result<bool, TemporalSeriesError> = sut.stationary_dickey_fuller_test(0.05);

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
