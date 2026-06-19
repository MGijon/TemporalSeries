use temporalseries::series::TimeSeries;

#[test]
#[allow(non_snake_case)]
fn test__given_valid_time_series_object__when__ask_for_len__returns_it_correctly() {
    // Given
    let sut_1: TimeSeries = TimeSeries::new(vec![1], vec![100.0]).unwrap();
    let sut_2: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![100.0, 110.0, 121.0]).unwrap();

    // When
    let result_sut_1: usize = sut_1.len();
    let result_sut_2: usize = sut_2.len();

    // Then
    assert!(result_sut_1 == 1);
    assert!(result_sut_2 == 3);
}
