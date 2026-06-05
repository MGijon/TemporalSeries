use temporalseries::series::TimeSeries;

#[test]
#[allow(non_snake_case)]
fn test__given_valid_series__when_rolling_mean_window_3__then_first_two_are_nan() {
    // Given
    let ts: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    // When
    let result: TimeSeries = ts.rolling(3).mean().unwrap();

    // Then
    assert!(result.values[0].is_nan());
    assert!(result.values[1].is_nan());
}

#[test]
#[allow(non_snake_case)]
fn test__given_valid_series__when_rolling_mean_window_3__then_computes_correctly() {
    // Given
    let ts: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    // When
    let result: TimeSeries = ts.rolling(3).mean().unwrap();

    // Then
    assert!((result.values[2] - 2.0).abs() < 1e-6); // (1 + 2 + 3) / 3
    assert!((result.values[3] - 3.0).abs() < 1e-6); // (2 + 3 + 4) / 3
    assert!((result.values[4] - 4.0).abs() < 1e-6); // (3 + 4 + 5) / 3
}

#[test]
#[allow(non_snake_case)]
fn test__given_valid_series__when_rolling_mean_window_1__then_equals_original() {
    // Given
    let ts: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![10.0, 20.0, 30.0]).unwrap();

    // When
    let result: TimeSeries = ts.rolling(1).mean().unwrap();

    // Then
    assert!((result.values[0] - 10.0).abs() < 1e-6);
    assert!((result.values[1] - 20.0).abs() < 1e-6);
    assert!((result.values[2] - 30.0).abs() < 1e-6);
}

#[test]
#[allow(non_snake_case)]
fn test__given_valid_series__when_rolling_mean__then_preserves_index() {
    // Given
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let ts: TimeSeries = TimeSeries::new(index.clone(), vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    // When
    let result: TimeSeries = ts.rolling(2).mean().unwrap();

    // Then
    assert_eq!(result.index, index);
}

#[test]
#[allow(non_snake_case)]
fn test__given_valid_series__when_rolling_mean__then_output_length_equals_input() {
    // Given
    let ts: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    // When
    let result: TimeSeries = ts.rolling(3).mean().unwrap();

    // Then
    assert_eq!(result.len(), ts.len());
}
