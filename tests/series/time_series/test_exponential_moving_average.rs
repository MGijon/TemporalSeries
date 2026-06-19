use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_ema__then_returns_constant() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![4.0, 4.0, 4.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.exponential_moving_average(3);

    // Then
    let ema: TimeSeries = result.unwrap();
    assert!(ema.values.iter().all(|&v| v == 4.0));
}

#[test]
#[allow(non_snake_case)]
fn test__given_linear_series_span_3__when_compute_ema__then_computes_correctly() {
    // Given
    // span=3 -> alpha = 2/(3+1) = 0.5
    // EMA[0] = 1.0
    // EMA[1] = 0.5*2 + 0.5*1.0 = 1.5
    // EMA[2] = 0.5*3 + 0.5*1.5 = 2.25
    // EMA[3] = 0.5*4 + 0.5*2.25 = 3.125
    // EMA[4] = 0.5*5 + 0.5*3.125 = 4.0625
    let sut: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.exponential_moving_average(3);

    // Then
    let ema: TimeSeries = result.unwrap();
    assert!((ema.values[0] - 1.0).abs() < 1e-9);
    assert!((ema.values[1] - 1.5).abs() < 1e-9);
    assert!((ema.values[2] - 2.25).abs() < 1e-9);
    assert!((ema.values[3] - 3.125).abs() < 1e-9);
    assert!((ema.values[4] - 4.0625).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_empty_series__when_compute_ema__then_returns_empty_series_error() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![], vec![]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.exponential_moving_average(3);

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
