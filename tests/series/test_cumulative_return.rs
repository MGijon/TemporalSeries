use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_cumulative_return__then_returns_zero() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![5.0, 5.0, 5.0]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.cumulative_return();

    // Then
    assert!(result.unwrap() == 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_growing_series__when_compute_cumulative_return__then_computes_correctly() {
    // Given
    // (121 - 100) / 100 = 0.21
    let sut: TimeSeries =
        TimeSeries::new(vec![1, 2, 3], vec![100.0, 110.0, 121.0]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.cumulative_return();

    // Then
    assert!((result.unwrap() - 0.21).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_halving_series__when_compute_cumulative_return__then_returns_minus_half() {
    // Given
    // (50 - 100) / 100 = -0.5
    let sut: TimeSeries = TimeSeries::new(vec![1, 2], vec![100.0, 50.0]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.cumulative_return();

    // Then
    assert!((result.unwrap() - (-0.5)).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_empty_series__when_compute_cumulative_return__then_returns_empty_series_error() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![], vec![]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.cumulative_return();

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
