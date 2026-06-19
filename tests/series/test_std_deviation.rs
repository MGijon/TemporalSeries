use temporalseries::series::TimeSeries;

#[test]
#[allow(non_snake_case)]
fn test__given_valid_time_series_object__when_compute_std__then_computes_it_correctly() {
    // Given
    let sut_1: TimeSeries = TimeSeries::new(vec![1], vec![0.0]).unwrap();
    let sut_2: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![0.0, 0.0, 0.0]).unwrap();
    // TODO: compute this by hand for a few examples and add more suts to the test suite
    //let sut_3: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![1.0, 2.0, 3.0]).unwrap();

    // When
    let result_sut_1: f64 = sut_1.std_deviation();
    let result_sut_2: f64 = sut_2.std_deviation();
    //let result_sut_3: f64 = sut_3.std_deviation();

    // Then
    assert!(result_sut_1 == 0.0);
    assert!(result_sut_2 == 0.0);
    //assert!(result_sut_3 == 0.0);
}
