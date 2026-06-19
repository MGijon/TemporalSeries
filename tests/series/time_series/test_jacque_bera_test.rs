use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_near_normal_series__when_compute_jacque_bera_test__then_returns_true() {
    // Given
    // [1,2,3,4,5]: skewness=0, excess_kurtosis=-1.3
    // JB = 5*(0/6 + 1.69/24) = 0.352 < 5.991 -> fail to reject normality
    let sut: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    // When
    let result: Result<bool, TemporalSeriesError> = sut.jacque_bera_test(0.05);

    // Then
    assert!(result.unwrap());
}

#[test]
#[allow(non_snake_case)]
fn test__given_highly_skewed_series__when_compute_jacque_bera_test__then_returns_false() {
    // Given
    // [1,1,1,1,1,1,1,1,1,20]: strong right skew, heavy tail -> JB >> 5.991 -> reject normality
    let sut: TimeSeries = TimeSeries::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 20.0],
    )
    .unwrap();

    // When
    let result: Result<bool, TemporalSeriesError> = sut.jacque_bera_test(0.05);

    // Then
    assert!(!result.unwrap());
}

#[test]
#[allow(non_snake_case)]
fn test__given_series_too_short__when_compute_jacque_bera_test__then_returns_error() {
    // Given
    // jacque_bera_statistics requires n >= 4 (excess_kurtosis requirement)
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![1.0, 2.0, 3.0]).unwrap();

    // When
    let result: Result<bool, TemporalSeriesError> = sut.jacque_bera_test(0.05);

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::EmptySeries)));
}
