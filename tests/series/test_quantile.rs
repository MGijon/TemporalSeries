use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_all_zeros_series__when_compute_quantile__then_returns_zero() {
    // Given
    let sut_1: TimeSeries = TimeSeries::new(vec![1], vec![0.0]).unwrap();
    let sut_2: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![0.0, 0.0, 0.0]).unwrap();

    // When
    let result_sut_1_p25: Result<f64, TemporalSeriesError> = sut_1.quantile(0.25);
    let result_sut_1_p50: Result<f64, TemporalSeriesError> = sut_1.quantile(0.50);
    let result_sut_1_p75: Result<f64, TemporalSeriesError> = sut_1.quantile(0.75);

    let result_sut_2_p25: Result<f64, TemporalSeriesError> = sut_2.quantile(0.25);
    let result_sut_2_p50: Result<f64, TemporalSeriesError> = sut_2.quantile(0.50);
    let result_sut_2_p75: Result<f64, TemporalSeriesError> = sut_2.quantile(0.75);

    // Then
    assert!(result_sut_1_p25.unwrap() == 0.0);
    assert!(result_sut_1_p50.unwrap() == 0.0);
    assert!(result_sut_1_p75.unwrap() == 0.0);

    assert!(result_sut_2_p25.unwrap() == 0.0);
    assert!(result_sut_2_p50.unwrap() == 0.0);
    assert!(result_sut_2_p75.unwrap() == 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_ordered_series__when_compute_quantile__then_returns_correct_value() {
    // Given
    // [1.0, 2.0, 3.0, 4.0, 5.0], n=5
    // Linear interpolation: h = p * (n-1)
    // p=0.0  -> h=0.0 -> sorted[0] = 1.0
    // p=0.25 -> h=1.0 -> sorted[1] = 2.0
    // p=0.5  -> h=2.0 -> sorted[2] = 3.0
    // p=0.75 -> h=3.0 -> sorted[3] = 4.0
    // p=1.0  -> h=4.0 -> sorted[4] = 5.0
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    // When
    let result_p0:   Result<f64, TemporalSeriesError> = sut.quantile(0.0);
    let result_p25:  Result<f64, TemporalSeriesError> = sut.quantile(0.25);
    let result_p50:  Result<f64, TemporalSeriesError> = sut.quantile(0.5);
    let result_p75:  Result<f64, TemporalSeriesError> = sut.quantile(0.75);
    let result_p100: Result<f64, TemporalSeriesError> = sut.quantile(1.0);

    // Then
    assert!(result_p0.unwrap() == 1.0);
    assert!(result_p25.unwrap() == 2.0);
    assert!(result_p50.unwrap() == 3.0);
    assert!(result_p75.unwrap() == 4.0);
    assert!(result_p100.unwrap() == 5.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_even_length_series__when_compute_median__then_interpolates_correctly() {
    // Given
    // [1.0, 2.0, 3.0, 4.0], n=4
    // p=0.5 -> h=1.5 -> lo=1, hi=2, frac=0.5 -> 2.0 + 0.5*(3.0-2.0) = 2.5
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3, 4], vec![1.0, 2.0, 3.0, 4.0]).unwrap();

    // When
    let result_p50: Result<f64, TemporalSeriesError> = sut.quantile(0.5);

    // Then
    assert!((result_p50.unwrap() - 2.5).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_unordered_series__when_compute_quantile__then_sorts_before_computing() {
    // Given
    // Same values as ordered test but shuffled — result must be identical
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3, 4, 5], vec![5.0, 3.0, 1.0, 4.0, 2.0]).unwrap();

    // When
    let result_p0:   Result<f64, TemporalSeriesError> = sut.quantile(0.0);
    let result_p25:  Result<f64, TemporalSeriesError> = sut.quantile(0.25);
    let result_p50:  Result<f64, TemporalSeriesError> = sut.quantile(0.5);
    let result_p75:  Result<f64, TemporalSeriesError> = sut.quantile(0.75);
    let result_p100: Result<f64, TemporalSeriesError> = sut.quantile(1.0);

    // Then
    assert!(result_p0.unwrap() == 1.0);
    assert!(result_p25.unwrap() == 2.0);
    assert!(result_p50.unwrap() == 3.0);
    assert!(result_p75.unwrap() == 4.0);
    assert!(result_p100.unwrap() == 5.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_boundary_p_values__when_compute_quantile__then_returns_ok() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![1.0, 2.0, 3.0]).unwrap();

    // When
    let result_p0:   Result<f64, TemporalSeriesError> = sut.quantile(0.0);
    let result_p100: Result<f64, TemporalSeriesError> = sut.quantile(1.0);

    // Then
    assert!(result_p0.is_ok());
    assert!(result_p100.is_ok());
}

#[test]
#[allow(non_snake_case)]
fn test__given_invalid_p__when_compute_quantile__then_returns_parameter_range_error() {
    // Given
    let sut_1: TimeSeries = TimeSeries::new(vec![1], vec![0.0]).unwrap();
    let sut_2: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![0.0, 0.0, 0.0]).unwrap();

    // When
    let result_sut_1_below_0: Result<f64, TemporalSeriesError> = sut_1.quantile(-0.1);
    let result_sut_1_above_1: Result<f64, TemporalSeriesError> = sut_1.quantile(1.1);
    let result_sut_2_below_0: Result<f64, TemporalSeriesError> = sut_2.quantile(-0.1);
    let result_sut_2_above_1: Result<f64, TemporalSeriesError> = sut_2.quantile(1.1);

    // Then
    assert!(matches!(result_sut_1_below_0, Err(TemporalSeriesError::ParameterRangeError(_))));
    assert!(matches!(result_sut_1_above_1, Err(TemporalSeriesError::ParameterRangeError(_))));
    assert!(matches!(result_sut_2_below_0, Err(TemporalSeriesError::ParameterRangeError(_))));
    assert!(matches!(result_sut_2_above_1, Err(TemporalSeriesError::ParameterRangeError(_))));
}
