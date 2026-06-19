use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_symmetric_series__when_compute_skewness__then_returns_zero() {
    // Given
    // [1,2,3,4,5] is symmetric around 3; skewness = 0
    let sut: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.skewness();

    // Then
    assert!(result.unwrap().abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_right_skewed_series__when_compute_skewness__then_returns_positive() {
    // Given
    // [1,1,1,1,5]: heavy right tail -> positive skewness = 1.5
    // mean=1.8, m2=2.56, m3=6.144, skew=6.144/2.56^1.5=1.5
    let sut: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 1.0, 1.0, 1.0, 5.0]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.skewness();

    // Then
    assert!((result.unwrap() - 1.5).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_series_too_short__when_compute_skewness__then_returns_error() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![1, 2], vec![1.0, 2.0]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.skewness();

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
