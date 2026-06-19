use temporalseries::series::TimeSeries;

#[test]
#[allow(non_snake_case)]
fn test__given_non_empty_time_series_object__when_is_len__returns_false() {
    // Given
    let sut_1: TimeSeries = TimeSeries::new(vec![1], vec![100.0]).unwrap();
    let sut_2: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![100.0, 110.0, 121.0]).unwrap();

    // When
    let result_sut_1: bool = sut_1.is_empty();
    let result_sut_2: bool = sut_2.is_empty();

    // Then
    assert!(result_sut_1 == false);
    assert!(result_sut_2 == false);
}

#[test]
#[allow(non_snake_case)]
fn test__given_empty_time_series_object__when_is_len__returns_true() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![], vec![]).unwrap();

    // When
    let result: bool = sut.is_empty();

    // Then
    assert!(result == true);
}
