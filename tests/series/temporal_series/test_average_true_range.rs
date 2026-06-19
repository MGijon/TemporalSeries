use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_atr__then_returns_zeros() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4],
        ColumnarBackend::new(vec![5.0, 5.0, 5.0, 5.0]),
    )
    .unwrap();

    // When
    let result = sut.average_true_range(2).unwrap();
    let values: Vec<f64> = result.iter().copied().collect();

    // Then
    assert!(values[0].is_nan());
    assert!(values[1].is_nan());
    assert_eq!(values[2], 0.0);
    assert_eq!(values[3], 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_growing_series_window_2__when_compute_atr__then_computes_correctly() {
    // Given
    // TR = [NaN, 2, 3, 4]; ATR(2): mean(2,3)=2.5, mean(3,4)=3.5
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4],
        ColumnarBackend::new(vec![1.0, 3.0, 6.0, 10.0]),
    )
    .unwrap();

    // When
    let result = sut.average_true_range(2).unwrap();
    let values: Vec<f64> = result.iter().copied().collect();

    // Then
    assert!((values[2] - 2.5).abs() < 1e-9);
    assert!((values[3] - 3.5).abs() < 1e-9);
}
