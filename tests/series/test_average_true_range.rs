use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_constant_series__when_compute_average_true_range__then_returns_zeros() {
    // Given
    let sut: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4], vec![5.0, 5.0, 5.0, 5.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.average_true_range(2);

    // Then
    let atr: TimeSeries = result.unwrap();
    // TR = [NaN, 0, 0, 0]; ATR(2): positions 0,1 = NaN, positions 2,3 = 0
    assert!(atr.values[0].is_nan());
    assert!(atr.values[1].is_nan());
    assert!(atr.values[2] == 0.0);
    assert!(atr.values[3] == 0.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_growing_series_window_2__when_compute_average_true_range__then_computes_correctly()
{
    // Given
    // [1, 3, 6, 10] -> TR = [NaN, 2, 3, 4]
    // ATR(2): t=2: mean(2,3)=2.5, t=3: mean(3,4)=3.5
    let sut: TimeSeries =
        TimeSeries::new(vec![1, 2, 3, 4], vec![1.0, 3.0, 6.0, 10.0]).unwrap();

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = sut.average_true_range(2);

    // Then
    let atr: TimeSeries = result.unwrap();
    assert!(atr.values[0].is_nan());
    assert!(atr.values[1].is_nan());
    assert!((atr.values[2] - 2.5).abs() < 1e-9);
    assert!((atr.values[3] - 3.5).abs() < 1e-9);
}
