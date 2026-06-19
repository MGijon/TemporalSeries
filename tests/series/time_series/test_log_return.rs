use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_log_return__then_all_returns_are_zero() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![5.0, 5.0, 5.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.log_return();

    // Then
    let r: TimeSeries = result.unwrap();
    assert!(r.values[0].is_nan());
    assert!(r.values[1] == 0.0);
    assert!(r.values[2] == 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_series_with_e_ratio__when_compute_log_return__then_returns_one() {
    // Given
    // ln(e / 1) = 1.0
    let sut: TimeSeries = TimeSeries::new(vec![1, 2], vec![1.0, std::f64::consts::E]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.log_return();

    // Then
    let r: TimeSeries = result.unwrap();
    assert!(r.values[0].is_nan());
    assert!((r.values[1] - 1.0).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_growing_series__when_compute_log_return__then_computes_correctly() {
    // Given
    // [100, 110] -> r_1 = ln(110/100) = ln(1.1)
    let sut: TimeSeries = TimeSeries::new(vec![1, 2], vec![100.0, 110.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.log_return();

    // Then
    let r: TimeSeries = result.unwrap();
    assert!(r.values[0].is_nan());
    assert!((r.values[1] - (1.1_f64).ln()).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_empty_series__when_compute_log_return__then_returns_empty_series_error() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![], vec![]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.log_return();

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
