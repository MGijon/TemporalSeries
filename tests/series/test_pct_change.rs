use temporalseries::series::TimeSeries;

#[test]
#[allow(non_snake_case)]
fn test__given_valid_temporal_series_object__when_compute_pct_change__then_computes_correctly() {
    // Given
    let time_serie = TimeSeries::new(vec![1, 2, 3], vec![100.0, 110.0, 121.0]);

    // When
    let returns = time_serie.pct_change();

    // Then
    assert!((returns.values[1] - 0.10).abs() < 1e-6);
}
