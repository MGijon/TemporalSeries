use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_any_series__when_compute_acf_lag_0__then_returns_one() {
    // Given
    let sut_1: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![1.0, 2.0, 3.0]).unwrap();
    let sut_2: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![5.0, 2.0, 8.0, 1.0, 4.0]).unwrap();

    // When
    let result_1: Result<f64, TemporalSeriesError> = sut_1.autocorrelation_function(0);
    let result_2: Result<f64, TemporalSeriesError> = sut_2.autocorrelation_function(0);

    // Then
    assert!((result_1.unwrap() - 1.0).abs() < 1e-9);
    assert!((result_2.unwrap() - 1.0).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_linear_series__when_compute_acf_lag_1__then_computes_correctly() {
    // Given
    // [1,2,3,4,5], mean=3, variance=10
    // ACF(1) = [(-1)(-2)+(0)(-1)+(1)(0)+(2)(1)] / 10 = 4/10 = 0.4
    let sut: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.autocorrelation_function(1);

    // Then
    assert!((result.unwrap() - 0.4).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_acf__then_returns_nan() {
    // Given
    // Zero variance -> ACF is NaN
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![7.0, 7.0, 7.0]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.autocorrelation_function(1);

    // Then
    assert!(result.unwrap().is_nan());
}

#[test]
#[allow(non_snake_case)]
fn test__given_lag_out_of_range__when_compute_acf__then_returns_error() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![1.0, 2.0, 3.0]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.autocorrelation_function(5);

    // Then
    assert!(matches!(
        result,
        Err(TemporalSeriesError::ParameterRangeError(_))
    ));
}
