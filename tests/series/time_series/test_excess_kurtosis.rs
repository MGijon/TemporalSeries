use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_linear_series__when_compute_excess_kurtosis__then_computes_correctly() {
    // Given
    // [1,2,3,4,5]: m2=2.0, m4=6.8
    // excess_kurtosis = 6.8 / 2.0^2 - 3 = 1.7 - 3 = -1.3
    let sut: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.excess_kurtosis();

    // Then
    assert!((result.unwrap() - (-1.3)).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_series_too_short__when_compute_excess_kurtosis__then_returns_error() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![1.0, 2.0, 3.0]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.excess_kurtosis();

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
