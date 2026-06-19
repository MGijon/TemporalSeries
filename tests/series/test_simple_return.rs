use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_simple_return__then_all_returns_are_zero() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![5.0, 5.0, 5.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.simple_return();

    // Then
    let r: TimeSeries = result.unwrap();
    assert!(r.values[0].is_nan());
    assert!(r.values[1] == 0.0);
    assert!(r.values[2] == 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_growing_series__when_compute_simple_return__then_computes_correctly() {
    // Given
    // [100, 110, 121] -> r_1 = (110-100)/100 = 0.1, r_2 = (121-110)/110 = 0.1
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![100.0, 110.0, 121.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.simple_return();

    // Then
    let r: TimeSeries = result.unwrap();
    assert!(r.values[0].is_nan());
    assert!((r.values[1] - 0.1).abs() < 1e-9);
    assert!((r.values[2] - 0.1).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_halving_series__when_compute_simple_return__then_returns_minus_half() {
    // Given
    // [100, 50] -> r_1 = (50 - 100) / 100 = -0.5
    let sut: TimeSeries = TimeSeries::new(vec![1, 2], vec![100.0, 50.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.simple_return();

    // Then
    let r: TimeSeries = result.unwrap();
    assert!(r.values[0].is_nan());
    assert!((r.values[1] - (-0.5)).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_empty_series__when_compute_simple_return__then_returns_empty_series_error() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![], vec![]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.simple_return();

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
