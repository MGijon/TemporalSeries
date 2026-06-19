use temporalseries::errors::TemporalSeriesError;
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_true_range__then_returns_zeros() {
    // Given
    let sut: TS =
        TemporalSeries::new(vec![1, 2, 3], ColumnarBackend::new(vec![5.0, 5.0, 5.0])).unwrap();

    // When
    let result = sut.true_range().unwrap();
    let values: Vec<f64> = result.iter().copied().collect();

    // Then
    assert!(values[0].is_nan());
    assert_eq!(values[1], 0.0);
    assert_eq!(values[2], 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_growing_series__when_compute_true_range__then_computes_correctly() {
    // Given
    // |3-1|=2, |6-3|=3, |10-6|=4
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4],
        ColumnarBackend::new(vec![1.0, 3.0, 6.0, 10.0]),
    )
    .unwrap();

    // When
    let result = sut.true_range().unwrap();
    let values: Vec<f64> = result.iter().copied().collect();

    // Then
    assert!(values[0].is_nan());
    assert!((values[1] - 2.0).abs() < 1e-9);
    assert!((values[2] - 3.0).abs() < 1e-9);
    assert!((values[3] - 4.0).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_empty_series__when_compute_true_range__then_returns_empty_series_error() {
    // Given
    let sut: TS = TemporalSeries::new(vec![], ColumnarBackend::new(vec![])).unwrap();

    // When
    let result: Result<_, TemporalSeriesError> = sut.true_range();

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
