use temporalseries::series::TimeSeries;

#[test]
#[allow(non_snake_case)]
fn test__given_valid_time_series_object__when_compute_all_quantiles__then_returns_it_correctly() {
    // Given
    let sut_1: TimeSeries = TimeSeries::new(vec![1], vec![0.0]).unwrap();
    let sut_2: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![0.0, 0.0, 0.0]).unwrap();

    // When

    // Then
}
