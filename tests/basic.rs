use temporalseries::series::TimeSeries;

#[test]
fn test_pct_change() {
    // Given
    let time_serie = TimeSeries::new(vec![1, 2, 3], vec![100.0, 110.0, 121.0]);
    let returns = time_serie.pct_change();

    assert!((returns.values[1] - 0.10).abs() < 1e-6);
}
