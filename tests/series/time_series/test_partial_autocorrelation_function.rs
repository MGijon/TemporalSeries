use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_any_series__when_compute_pacf_lag_0__then_returns_one() {
    // Given
    let sut: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.partial_autocorrelation_function(0);

    // Then
    assert!((result.unwrap() - 1.0).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_series__when_compute_pacf_lag_1__then_equals_acf_lag_1() {
    // Given
    // PACF(1) == ACF(1) by definition
    // For [1,2,3,4,5]: ACF(1) = 0.4
    let sut: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    // When
    let result_pacf: Result<f64, TemporalSeriesError> = sut.partial_autocorrelation_function(1);
    let result_acf: Result<f64, TemporalSeriesError> = sut.autocorrelation_function(1);

    // Then
    assert!((result_pacf.unwrap() - result_acf.unwrap()).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_lag_out_of_range__when_compute_pacf__then_returns_error() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![1.0, 2.0, 3.0]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.partial_autocorrelation_function(5);

    // Then
    assert!(matches!(
        result,
        Err(TemporalSeriesError::ParameterRangeError(_))
    ));
}
