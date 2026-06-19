use temporalseries::errors::TemporalSeriesError;
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_odd_length_series__when_compute_quartiles__then_returns_exact_values() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5],
        ColumnarBackend::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]),
    )
    .unwrap();

    // When / Then
    assert_eq!(sut.quantile(0.0).unwrap(), 1.0);
    assert_eq!(sut.quantile(0.25).unwrap(), 2.0);
    assert_eq!(sut.quantile(0.5).unwrap(), 3.0);
    assert_eq!(sut.quantile(0.75).unwrap(), 4.0);
    assert_eq!(sut.quantile(1.0).unwrap(), 5.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_even_length_series__when_compute_median__then_interpolates() {
    // Given
    // h = 0.5 * 3 = 1.5  ->  2.0 + 0.5 * (3.0 - 2.0) = 2.5
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4],
        ColumnarBackend::new(vec![1.0, 2.0, 3.0, 4.0]),
    )
    .unwrap();

    // When
    let result: f64 = sut.quantile(0.5).unwrap();

    // Then
    assert!((result - 2.5).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_out_of_range_p__when_compute_quantile__then_returns_parameter_range_error() {
    // Given
    let sut: TS =
        TemporalSeries::new(vec![1, 2, 3], ColumnarBackend::new(vec![1.0, 2.0, 3.0])).unwrap();

    // When
    let result_low: Result<f64, TemporalSeriesError> = sut.quantile(-0.1);
    let result_high: Result<f64, TemporalSeriesError> = sut.quantile(1.1);

    // Then
    assert!(matches!(
        result_low,
        Err(TemporalSeriesError::ParameterRangeError(_))
    ));
    assert!(matches!(
        result_high,
        Err(TemporalSeriesError::ParameterRangeError(_))
    ));
}
