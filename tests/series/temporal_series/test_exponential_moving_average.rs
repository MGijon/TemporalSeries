use temporalseries::errors::TemporalSeriesError;
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_ema__then_returns_constant() {
    // Given
    let sut: TS =
        TemporalSeries::new(vec![1, 2, 3], ColumnarBackend::new(vec![5.0, 5.0, 5.0])).unwrap();

    // When
    let result = sut.exponential_moving_average(3).unwrap();
    let values: Vec<f64> = result.iter().copied().collect();

    // Then
    assert_eq!(values[0], 5.0);
    assert_eq!(values[1], 5.0);
    assert_eq!(values[2], 5.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_span_3__when_compute_ema__then_computes_with_alpha_0_5() {
    // Given
    // span=3 -> alpha=0.5; EMA_0=1, EMA_1=0.5*2+0.5*1=1.5, EMA_2=0.5*3+0.5*1.5=2.25
    let sut: TS =
        TemporalSeries::new(vec![1, 2, 3], ColumnarBackend::new(vec![1.0, 2.0, 3.0])).unwrap();

    // When
    let result = sut.exponential_moving_average(3).unwrap();
    let values: Vec<f64> = result.iter().copied().collect();

    // Then
    assert_eq!(values[0], 1.0);
    assert_eq!(values[1], 1.5);
    assert_eq!(values[2], 2.25);
}

#[test]
#[allow(non_snake_case)]
fn test__given_empty_series__when_compute_ema__then_returns_empty_series_error() {
    // Given
    let sut: TS = TemporalSeries::new(vec![], ColumnarBackend::new(vec![])).unwrap();

    // When
    let result: Result<_, TemporalSeriesError> = sut.exponential_moving_average(3);

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
