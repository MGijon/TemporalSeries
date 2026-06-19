use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};

#[test]
#[allow(non_snake_case)]
fn test__given_all_zeros__when_compute_quantile__then_returns_zero() {
    let sut_1 = TimeSeries::new(vec![1], vec![0.0]).unwrap();
    let sut_2 = TimeSeries::new(vec![1, 2, 3], vec![0.0, 0.0, 0.0]).unwrap();

    for p in [0.25, 0.50, 0.75, 0.95, 0.99] {
        assert_eq!(sut_1.quantile(p).unwrap(), 0.0);
        assert_eq!(sut_2.quantile(p).unwrap(), 0.0);
    }
}

#[test]
#[allow(non_snake_case)]
fn test__given_ordered_series__when_compute_quantile__then_returns_correct_value() {
    // [1.0, 2.0, 3.0, 4.0, 5.0] — n=5, indices 0..4
    // Linear interpolation: h = p * (n-1)
    let ts = TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();

    assert_eq!(ts.quantile(0.0).unwrap(), 1.0);  // h=0.0  → sorted[0]
    assert_eq!(ts.quantile(0.25).unwrap(), 2.0); // h=1.0  → sorted[1]
    assert_eq!(ts.quantile(0.5).unwrap(), 3.0);  // h=2.0  → sorted[2]
    assert_eq!(ts.quantile(0.75).unwrap(), 4.0); // h=3.0  → sorted[3]
    assert_eq!(ts.quantile(1.0).unwrap(), 5.0);  // h=4.0  → sorted[4]
}

#[test]
#[allow(non_snake_case)]
fn test__given_series__when_compute_median__then_interpolates_correctly() {
    // Even-length series: [1.0, 2.0, 3.0, 4.0] — n=4
    // p=0.5 → h=1.5 → lo=1, hi=2, frac=0.5 → 2.0 + 0.5*(3.0-2.0) = 2.5
    let ts = TimeSeries::new(vec![1, 2, 3, 4], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    assert!((ts.quantile(0.5).unwrap() - 2.5).abs() < 1e-9);
}

#[test]
#[allow(non_snake_case)]
fn test__given_unordered_series__when_compute_quantile__then_sorts_before_computing() {
    // Same values as the ordered test but shuffled — result must be identical.
    let ts = TimeSeries::new(vec![1, 2, 3, 4, 5], vec![5.0, 3.0, 1.0, 4.0, 2.0]).unwrap();

    assert_eq!(ts.quantile(0.0).unwrap(), 1.0);
    assert_eq!(ts.quantile(0.25).unwrap(), 2.0);
    assert_eq!(ts.quantile(0.5).unwrap(), 3.0);
    assert_eq!(ts.quantile(0.75).unwrap(), 4.0);
    assert_eq!(ts.quantile(1.0).unwrap(), 5.0);
}

#[test]
#[allow(non_snake_case)]
fn test__given_valid_p_boundary_values__when_compute_quantile__then_returns_ok() {
    let ts = TimeSeries::new(vec![1, 2, 3], vec![1.0, 2.0, 3.0]).unwrap();

    assert!(ts.quantile(0.0).is_ok());
    assert!(ts.quantile(1.0).is_ok());
}

#[test]
#[allow(non_snake_case)]
fn test__given_invalid_p__when_compute_quantile__then_returns_parameter_range_error() {
    let sut_1 = TimeSeries::new(vec![1], vec![0.0]).unwrap();
    let sut_2 = TimeSeries::new(vec![1, 2, 3], vec![0.0, 0.0, 0.0]).unwrap();

    let result_1_below = sut_1.quantile(-0.1);
    let result_1_above = sut_1.quantile(1.1);
    let result_2_below = sut_2.quantile(-0.1);
    let result_2_above = sut_2.quantile(1.1);

    assert!(matches!(result_1_below, Err(TemporalSeriesError::ParameterRangeError(_))));
    assert!(matches!(result_1_above, Err(TemporalSeriesError::ParameterRangeError(_))));
    assert!(matches!(result_2_below, Err(TemporalSeriesError::ParameterRangeError(_))));
    assert!(matches!(result_2_above, Err(TemporalSeriesError::ParameterRangeError(_))));
}
