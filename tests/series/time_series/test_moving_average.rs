use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_moving_average__then_returns_constant() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![4.0, 4.0, 4.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.moving_average(2);

    // Then
    let ma: TimeSeries = result.unwrap();
    assert!(ma.values[0].is_nan());
    assert!(ma.values[1] == 4.0);
    assert!(ma.values[2] == 4.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_linear_series_window_3__when_compute_moving_average__then_computes_correctly() {
    // Given
    // [1,2,3,4,5], window=3
    // MA[2] = (1+2+3)/3 = 2.0
    // MA[3] = (2+3+4)/3 = 3.0
    // MA[4] = (3+4+5)/3 = 4.0
    let sut: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.moving_average(3);

    // Then
    let ma: TimeSeries = result.unwrap();
    assert!(ma.values[0].is_nan());
    assert!(ma.values[1].is_nan());
    assert!((ma.values[2] - 2.0).abs() < 1e-9);
    assert!((ma.values[3] - 3.0).abs() < 1e-9);
    assert!((ma.values[4] - 4.0).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_window_larger_than_series__when_compute_moving_average__then_returns_error() {
    // Given
    let sut: TimeSeries = TimeSeries::new(vec![1, 2], vec![1.0, 2.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.moving_average(5);

    // Then
    assert!(matches!(
        result,
        Err(TemporalSeriesError::InvalidWindow { .. })
    ));
}
