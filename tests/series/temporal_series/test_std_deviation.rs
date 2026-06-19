use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

type TS = TemporalSeries<f64, ColumnarBackend<f64>>;

#[test]
#[allow(non_snake_case)]
fn test__given_single_element__when_compute_std_deviation__then_returns_zero() {
    // Given
    let sut: TS = TemporalSeries::new(vec![1], ColumnarBackend::new(vec![42.0])).unwrap();

    // When
    let result: f64 = sut.std_deviation();

    // Then
    assert_eq!(result, 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_std_deviation__then_returns_zero() {
    // Given
    let sut: TS =
        TemporalSeries::new(vec![1, 2, 3], ColumnarBackend::new(vec![5.0, 5.0, 5.0])).unwrap();

    // When
    let result: f64 = sut.std_deviation();

    // Then
    assert_eq!(result, 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_two_element_series__when_compute_std_deviation__then_returns_bessel_corrected_variance()
 {
    // Given
    // mean = 1.5, deviations = [-0.5, 0.5], variance = (0.25+0.25)/(2-1) = 0.5
    let sut: TS = TemporalSeries::new(vec![1, 2], ColumnarBackend::new(vec![1.0, 2.0])).unwrap();

    // When
    let result: f64 = sut.std_deviation();

    // Then
    assert!((result - 0.5).abs() < 1e-9);
}
