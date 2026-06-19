use temporalseries::errors::TemporalSeriesError;
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series_window_3__when_compute_moving_average__then_returns_constant() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5],
        ColumnarBackend::new(vec![4.0, 4.0, 4.0, 4.0, 4.0]),
    )
    .unwrap();

    // When
    let result = sut.moving_average(3).unwrap();
    let values: Vec<f64> = result.iter().copied().collect();

    // Then
    assert!(values[0].is_nan());
    assert!(values[1].is_nan());
    assert_eq!(values[2], 4.0);
    assert_eq!(values[3], 4.0);
    assert_eq!(values[4], 4.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_linear_series_window_3__when_compute_moving_average__then_computes_correctly() {
    // Given
    // windows: [1,2,3]=2, [2,3,4]=3, [3,4,5]=4
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5],
        ColumnarBackend::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]),
    )
    .unwrap();

    // When
    let result = sut.moving_average(3).unwrap();
    let values: Vec<f64> = result.iter().copied().collect();

    // Then
    assert!(values[0].is_nan());
    assert!(values[1].is_nan());
    assert!((values[2] - 2.0).abs() < 1e-9);
    assert!((values[3] - 3.0).abs() < 1e-9);
    assert!((values[4] - 4.0).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_window_exceeds_length__when_compute_moving_average__then_returns_invalid_window_error()
 {
    // Given
    let sut: TS = TemporalSeries::new(vec![1, 2], ColumnarBackend::new(vec![1.0, 2.0])).unwrap();

    // When
    let result: Result<_, TemporalSeriesError> = sut.moving_average(5);

    // Then
    assert!(matches!(
        result,
        Err(TemporalSeriesError::InvalidWindow { .. })
    ));
}
