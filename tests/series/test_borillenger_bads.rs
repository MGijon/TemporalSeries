use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_bollinger_bands__then_all_bands_collapse_onto_mean() {
    // Given
    let sut: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![3.0, 3.0, 3.0, 3.0, 3.0]).unwrap();

    // When
    let result: Result<(TimeSeries, TimeSeries, TimeSeries), TemporalSeriesError> =
        sut.bollinger_bands(3, 2.0);

    // Then
    let (upper, mid, lower): (TimeSeries, TimeSeries, TimeSeries) = result.unwrap();
    assert!((upper.values[2] - 3.0).abs() < 1e-9);
    assert!((mid.values[2] - 3.0).abs() < 1e-9);
    assert!((lower.values[2] - 3.0).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_linear_series__when_compute_bollinger_bands__then_upper_above_lower() {
    // Given
    let sut: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    // When
    let result: Result<(TimeSeries, TimeSeries, TimeSeries), TemporalSeriesError> =
        sut.bollinger_bands(3, 2.0);

    // Then
    let (upper, mid, lower): (TimeSeries, TimeSeries, TimeSeries) = result.unwrap();
    // For every position where the band is defined, upper >= mid >= lower
    for i in 2..5 {
        assert!(upper.values[i] >= mid.values[i]);
        assert!(mid.values[i] >= lower.values[i]);
    }
}

#[test]
#[allow(non_snake_case)]
fn test__given_invalid_window__when_compute_bollinger_bands__then_returns_error() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![1, 2], vec![1.0, 2.0]).unwrap();

    // When
    let result: Result<(TimeSeries, TimeSeries, TimeSeries), TemporalSeriesError> =
        sut.bollinger_bands(5, 2.0);

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::InvalidWindow { .. })));
}
