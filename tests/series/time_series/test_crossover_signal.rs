use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_crossover_signal__then_all_signals_are_zero() {
    // Given
    let sut: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![3.0, 3.0, 3.0, 3.0, 3.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.crossover_signal(2, 3);

    // Then
    let sig: TimeSeries = result.unwrap();
    assert!(sig.values.iter().all(|&v| v == 0.0));
}

#[test]
#[allow(non_snake_case)]
fn test__given_series_with_bullish_crossover__when_compute_crossover_signal__then_detects_plus_one()
{
    // Given
    // Falling then rising: fast MA (2) crosses above slow MA (3) partway through.
    // [3,2,1,2,3,4,5]: fast crosses slow after the trough.
    let sut: TimeSeries = TimeSeries::new(
        vec![1, 2, 3, 4, 5, 6, 7],
        vec![3.0, 2.0, 1.0, 2.0, 3.0, 4.0, 5.0],
    )
    .unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.crossover_signal(2, 3);

    // Then
    let sig: TimeSeries = result.unwrap();
    assert!(sig.values.iter().any(|&v| v == 1.0));
}

#[test]
#[allow(non_snake_case)]
fn test__given_fast_not_smaller_than_slow__when_compute_crossover_signal__then_returns_error() {
    // Given
    let sut: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    // When
    let result_equal: Result<TimeSeries, TemporalSeriesError> = sut.crossover_signal(3, 3);
    let result_fast_larger: Result<TimeSeries, TemporalSeriesError> = sut.crossover_signal(4, 2);

    // Then
    assert!(matches!(
        result_equal,
        Err(TemporalSeriesError::ParameterRangeError(_))
    ));
    assert!(matches!(
        result_fast_larger,
        Err(TemporalSeriesError::ParameterRangeError(_))
    ));
}
