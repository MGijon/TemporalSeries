use temporalseries::errors::TemporalSeriesError;
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_any_series__when_compute_pacf_at_lag_0__then_returns_one() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5],
        ColumnarBackend::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]),
    )
    .unwrap();

    // When
    let result: f64 = sut.partial_autocorrelation_function(0).unwrap();

    // Then
    assert_eq!(result, 1.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_linear_series__when_compute_pacf_at_lag_1__then_equals_acf_at_lag_1() {
    // Given
    let sut: TS = TemporalSeries::new(
        vec![1, 2, 3, 4, 5],
        ColumnarBackend::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]),
    )
    .unwrap();

    // When
    let acf: f64 = sut.autocorrelation_function(1).unwrap();
    let pacf: f64 = sut.partial_autocorrelation_function(1).unwrap();

    // Then
    assert!((acf - pacf).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_lag_out_of_range__when_compute_pacf__then_returns_parameter_range_error() {
    // Given
    let sut: TS =
        TemporalSeries::new(vec![1, 2, 3], ColumnarBackend::new(vec![1.0, 2.0, 3.0])).unwrap();

    // When
    let result: Result<f64, TemporalSeriesError> = sut.partial_autocorrelation_function(10);

    // Then
    assert!(matches!(
        result,
        Err(TemporalSeriesError::ParameterRangeError(_))
    ));
}
