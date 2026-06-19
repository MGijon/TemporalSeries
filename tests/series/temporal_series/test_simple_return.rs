use temporalseries::errors::TemporalSeriesError;
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_simple_return__then_all_returns_are_zero() {
    // Given
    let sut: TS =
        TemporalSeries::new(vec![1, 2, 3], ColumnarBackend::new(vec![5.0, 5.0, 5.0])).unwrap();

    // When
    let result = sut.simple_return().unwrap();
    let values: Vec<f64> = result.iter().copied().collect();

    // Then
    assert!(values[0].is_nan());
    assert_eq!(values[1], 0.0);
    assert_eq!(values[2], 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_growing_series__when_compute_simple_return__then_computes_correctly() {
    // Given
    // [100, 110, 121] -> r_1 = (110-100)/100 = 0.1, r_2 = (121-110)/110 = 0.1
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3],
        ColumnarBackend::new(vec![100.0, 110.0, 121.0]),
    )
    .unwrap();

    // When
    let result = sut.simple_return().unwrap();
    let values: Vec<f64> = result.iter().copied().collect();

    // Then
    assert!(values[0].is_nan());
    assert!((values[1] - 0.1).abs() < 1e-9);
    assert!((values[2] - 0.1).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_empty_series__when_compute_simple_return__then_returns_empty_series_error() {
    // Given
    let sut: TS = TemporalSeries::new(vec![], ColumnarBackend::new(vec![])).unwrap();

    // When
    let result: Result<_, TemporalSeriesError> = sut.simple_return();

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
