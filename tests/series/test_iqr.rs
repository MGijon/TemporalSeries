use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_all_zeros_series__when_compute_iqr__then_returns_zero() {
    // Given
    let sut_1: TimeSeries = TimeSeries::new(vec![1], vec![0.0]).unwrap();
    let sut_2: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![0.0, 0.0, 0.0]).unwrap();

    // When
    let result_sut_1: Result<f64, TemporalSeriesError> = sut_1.iqr();
    let result_sut_2: Result<f64, TemporalSeriesError> = sut_2.iqr();

    // Then
    assert!(result_sut_1.unwrap() == 0.0);
    assert!(result_sut_2.unwrap() == 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_odd_length_series__when_compute_iqr__then_returns_correct_value() {
    // Given
    // [1.0, 2.0, 3.0, 4.0, 5.0], n=5
    // Q1: h = 0.25 * 4 = 1.0 -> sorted[1] = 2.0
    // Q3: h = 0.75 * 4 = 3.0 -> sorted[3] = 4.0
    // IQR = 4.0 - 2.0 = 2.0
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.iqr();

    // Then
    assert!(result.unwrap() == 2.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_even_length_series__when_compute_iqr__then_interpolates_correctly() {
    // Given
    // [1.0, 2.0, 3.0, 4.0], n=4
    // Q1: h = 0.25 * 3 = 0.75 -> 1.0 + 0.75*(2.0-1.0) = 1.75
    // Q3: h = 0.75 * 3 = 2.25 -> 3.0 + 0.25*(4.0-3.0) = 3.25
    // IQR = 3.25 - 1.75 = 1.5
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3, 4], vec![1.0, 2.0, 3.0, 4.0]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.iqr();

    // Then
    assert!((result.unwrap() - 1.5).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_unordered_series__when_compute_iqr__then_returns_correct_value() {
    // Given
    // Same values as the ordered test but shuffled — result must be identical
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3, 4, 5], vec![5.0, 3.0, 1.0, 4.0, 2.0]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.iqr();

    // Then
    assert!(result.unwrap() == 2.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_empty_series__when_compute_iqr__then_returns_empty_series_error() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![], vec![]).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.iqr();

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
