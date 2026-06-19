use temporalseries::errors::TemporalSeriesError;
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_bollinger_bands__then_all_bands_collapse() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5],
        ColumnarBackend::new(vec![3.0, 3.0, 3.0, 3.0, 3.0]),
    )
    .unwrap();

    // When
    let (upper, mid, lower) = sut.bollinger_bands(3, 2.0).unwrap();
    let u: Vec<f64> = upper.iter().copied().collect();
    let m: Vec<f64> = mid.iter().copied().collect();
    let l: Vec<f64> = lower.iter().copied().collect();

    // Then
    assert_eq!(u[2], 3.0);
    assert_eq!(m[2], 3.0);
    assert_eq!(l[2], 3.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_linear_series__when_compute_bollinger_bands__then_upper_ge_mid_ge_lower() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5],
        ColumnarBackend::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]),
    )
    .unwrap();

    // When
    let (upper, mid, lower) = sut.bollinger_bands(3, 2.0).unwrap();
    let u: Vec<f64> = upper.iter().copied().collect();
    let m: Vec<f64> = mid.iter().copied().collect();
    let l: Vec<f64> = lower.iter().copied().collect();

    // Then
    for i in 2..5 {
        assert!(u[i] >= m[i]);
        assert!(m[i] >= l[i]);
    }
}

#[test]
#[allow(non_snake_case)]
fn test__given_invalid_window__when_compute_bollinger_bands__then_returns_invalid_window_error() {
    // Given
    let sut: TS = TemporalSeries::new(vec![1, 2], ColumnarBackend::new(vec![1.0, 2.0])).unwrap();

    // When
    let result: Result<_, TemporalSeriesError> = sut.bollinger_bands(5, 2.0);

    // Then
    assert!(matches!(
        result,
        Err(TemporalSeriesError::InvalidWindow { .. })
    ));
}
