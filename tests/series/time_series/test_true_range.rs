use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_true_range__then_returns_zeros() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![5.0, 5.0, 5.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.true_range();

    // Then
    let tr: TimeSeries = result.unwrap();
    assert!(tr.values[0].is_nan());
    assert!(tr.values[1] == 0.0);
    assert!(tr.values[2] == 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_growing_series__when_compute_true_range__then_computes_correctly() {
    // Given
    // |3-1|=2, |6-3|=3, |10-6|=4
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3, 4], vec![1.0, 3.0, 6.0, 10.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.true_range();

    // Then
    let tr: TimeSeries = result.unwrap();
    assert!(tr.values[0].is_nan());
    assert!((tr.values[1] - 2.0).abs() < 1e-9);
    assert!((tr.values[2] - 3.0).abs() < 1e-9);
    assert!((tr.values[3] - 4.0).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_falling_series__when_compute_true_range__then_returns_absolute_values() {
    // Given
    // True range is always non-negative: |4-10|=6, |2-4|=2
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![10.0, 4.0, 2.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.true_range();

    // Then
    let tr: TimeSeries = result.unwrap();
    assert!(tr.values[0].is_nan());
    assert!((tr.values[1] - 6.0).abs() < 1e-9);
    assert!((tr.values[2] - 2.0).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_empty_series__when_compute_true_range__then_returns_empty_series_error() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![], vec![]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.true_range();

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
