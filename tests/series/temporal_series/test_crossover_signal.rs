use temporalseries::errors::TemporalSeriesError;
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_crossover_signal__then_all_zeros() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5],
        ColumnarBackend::new(vec![3.0, 3.0, 3.0, 3.0, 3.0]),
    )
    .unwrap();

    // When
    let result = sut.crossover_signal(2, 3).unwrap();

    // Then
    assert!(result.iter().all(|&v| v == 0.0));
}

#[test]
#[allow(non_snake_case)]
fn test__given_bullish_crossover__when_compute_crossover_signal__then_returns_positive_one() {
    // Given
    // Fast(2) crosses above slow(3) when the series suddenly rises.
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        ColumnarBackend::new(vec![1.0, 1.0, 1.0, 1.0, 5.0, 5.0, 5.0, 5.0]),
    )
    .unwrap();

    // When
    let result = sut.crossover_signal(2, 3).unwrap();
    let signals: Vec<f64> = result.iter().copied().collect();

    // Then
    assert!(signals.contains(&1.0));
    assert!(!signals.contains(&-1.0));
}

#[test]
#[allow(non_snake_case)]
fn test__given_fast_ge_slow__when_compute_crossover_signal__then_returns_parameter_range_error() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5],
        ColumnarBackend::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]),
    )
    .unwrap();

    // When
    let result: Result<_, TemporalSeriesError> = sut.crossover_signal(3, 3);

    // Then
    assert!(matches!(
        result,
        Err(TemporalSeriesError::ParameterRangeError(_))
    ));
}
