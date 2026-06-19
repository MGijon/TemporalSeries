use temporalseries::panel::Panel;

/// Compares two f64 slices element-wise, treating NaN as equal.
/// IEEE 754 mandates NaN != NaN, so plain `assert_eq!` would fail on any
/// series that uses NaN as a sentinel for "no value" (e.g. the first element
/// of `diff`, `simple_return`, or any rolling window result).
fn assert_f64_vecs_eq(left: &[f64], right: &[f64]) {
    assert_eq!(left.len(), right.len(), "slice lengths differ");
    for (i, (l, r)) in left.iter().zip(right.iter()).enumerate() {
        if l.is_nan() && r.is_nan() {
            continue;
        }
        assert_eq!(l, r, "mismatch at index {i}");
    }
}

/// 10-point panel with two symbols. Long enough for all analytical methods.
fn sample_panel() -> Panel {
    Panel::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        vec!["AAPL".into(), "MSFT".into()],
        vec![
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
            vec![2.0, 1.0, 3.0, 0.5, 4.0, -1.0, 5.0, 2.5, 3.5, 1.5],
        ],
    )
    .unwrap()
}

// ---------------------------------------------------------------------------
// Statistics
// ---------------------------------------------------------------------------

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_mean__then_matches_time_series_mean() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.mean();
    // Then
    assert_eq!(result["AAPL"], panel.get_series("AAPL").unwrap().mean());
    assert_eq!(result["MSFT"], panel.get_series("MSFT").unwrap().mean());
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_std_deviation__then_matches_time_series_std_deviation() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.std_deviation();
    // Then
    assert_eq!(
        result["AAPL"],
        panel.get_series("AAPL").unwrap().std_deviation()
    );
    assert_eq!(
        result["MSFT"],
        panel.get_series("MSFT").unwrap().std_deviation()
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_quantile__then_matches_time_series_quantile() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.quantile(0.5).unwrap();
    // Then
    assert_eq!(
        result["AAPL"],
        panel.get_series("AAPL").unwrap().quantile(0.5).unwrap()
    );
    assert_eq!(
        result["MSFT"],
        panel.get_series("MSFT").unwrap().quantile(0.5).unwrap()
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_iqr__then_matches_time_series_iqr() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.iqr().unwrap();
    // Then
    assert_eq!(
        result["AAPL"],
        panel.get_series("AAPL").unwrap().iqr().unwrap()
    );
    assert_eq!(
        result["MSFT"],
        panel.get_series("MSFT").unwrap().iqr().unwrap()
    );
}

// ---------------------------------------------------------------------------
// Returns
// ---------------------------------------------------------------------------

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_simple_return__then_matches_time_series_simple_return() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.simple_return().unwrap();
    // Then
    let aapl_expected = panel.get_series("AAPL").unwrap().simple_return().unwrap();
    assert_f64_vecs_eq(
        &result.get_series("AAPL").unwrap().values,
        &aapl_expected.values,
    );
    let msft_expected = panel.get_series("MSFT").unwrap().simple_return().unwrap();
    assert_f64_vecs_eq(
        &result.get_series("MSFT").unwrap().values,
        &msft_expected.values,
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_log_return__then_matches_time_series_log_return() {
    // Given — use positive values only so log_return is well-defined
    let panel = Panel::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        vec!["AAPL".into(), "MSFT".into()],
        vec![
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
            vec![2.0, 3.0, 5.0, 4.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0],
        ],
    )
    .unwrap();
    // When
    let result = panel.log_return().unwrap();
    // Then
    let aapl_expected = panel.get_series("AAPL").unwrap().log_return().unwrap();
    assert_f64_vecs_eq(
        &result.get_series("AAPL").unwrap().values,
        &aapl_expected.values,
    );
    let msft_expected = panel.get_series("MSFT").unwrap().log_return().unwrap();
    assert_f64_vecs_eq(
        &result.get_series("MSFT").unwrap().values,
        &msft_expected.values,
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_cumulative_return__then_matches_time_series_cumulative_return() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.cumulative_return().unwrap();
    // Then
    assert_eq!(
        result["AAPL"],
        panel
            .get_series("AAPL")
            .unwrap()
            .cumulative_return()
            .unwrap()
    );
    assert_eq!(
        result["MSFT"],
        panel
            .get_series("MSFT")
            .unwrap()
            .cumulative_return()
            .unwrap()
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_diff__then_matches_time_series_diff() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.diff().unwrap();
    // Then
    let aapl_expected = panel.get_series("AAPL").unwrap().diff().unwrap();
    assert_f64_vecs_eq(
        &result.get_series("AAPL").unwrap().values,
        &aapl_expected.values,
    );
    let msft_expected = panel.get_series("MSFT").unwrap().diff().unwrap();
    assert_f64_vecs_eq(
        &result.get_series("MSFT").unwrap().values,
        &msft_expected.values,
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_pct_change__then_matches_time_series_pct_change() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.pct_change().unwrap();
    // Then
    let aapl_expected = panel.get_series("AAPL").unwrap().pct_change().unwrap();
    assert_f64_vecs_eq(
        &result.get_series("AAPL").unwrap().values,
        &aapl_expected.values,
    );
    let msft_expected = panel.get_series("MSFT").unwrap().pct_change().unwrap();
    assert_f64_vecs_eq(
        &result.get_series("MSFT").unwrap().values,
        &msft_expected.values,
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_shift__then_matches_time_series_shift() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.shift(2).unwrap();
    // Then
    let aapl_expected = panel.get_series("AAPL").unwrap().shift(2).unwrap();
    assert_f64_vecs_eq(
        &result.get_series("AAPL").unwrap().values,
        &aapl_expected.values,
    );
    let msft_expected = panel.get_series("MSFT").unwrap().shift(2).unwrap();
    assert_f64_vecs_eq(
        &result.get_series("MSFT").unwrap().values,
        &msft_expected.values,
    );
}

// ---------------------------------------------------------------------------
// Moving averages
// ---------------------------------------------------------------------------

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_moving_average__then_matches_time_series_moving_average() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.moving_average(3).unwrap();
    // Then
    let aapl_expected = panel.get_series("AAPL").unwrap().moving_average(3).unwrap();
    assert_f64_vecs_eq(
        &result.get_series("AAPL").unwrap().values,
        &aapl_expected.values,
    );
    let msft_expected = panel.get_series("MSFT").unwrap().moving_average(3).unwrap();
    assert_f64_vecs_eq(
        &result.get_series("MSFT").unwrap().values,
        &msft_expected.values,
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_exponential_moving_average__then_matches_time_series_ema() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.exponential_moving_average(3).unwrap();
    // Then
    let aapl_expected = panel
        .get_series("AAPL")
        .unwrap()
        .exponential_moving_average(3)
        .unwrap();
    assert_f64_vecs_eq(
        &result.get_series("AAPL").unwrap().values,
        &aapl_expected.values,
    );
    let msft_expected = panel
        .get_series("MSFT")
        .unwrap()
        .exponential_moving_average(3)
        .unwrap();
    assert_f64_vecs_eq(
        &result.get_series("MSFT").unwrap().values,
        &msft_expected.values,
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_crossover_signal__then_matches_time_series_crossover_signal() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.crossover_signal(2, 4).unwrap();
    // Then
    let aapl_expected = panel
        .get_series("AAPL")
        .unwrap()
        .crossover_signal(2, 4)
        .unwrap();
    assert_f64_vecs_eq(
        &result.get_series("AAPL").unwrap().values,
        &aapl_expected.values,
    );
    let msft_expected = panel
        .get_series("MSFT")
        .unwrap()
        .crossover_signal(2, 4)
        .unwrap();
    assert_f64_vecs_eq(
        &result.get_series("MSFT").unwrap().values,
        &msft_expected.values,
    );
}

// ---------------------------------------------------------------------------
// Volatility
// ---------------------------------------------------------------------------

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_rolling_standard_deviation__then_matches_time_series_rolling_std() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.rolling_standard_deviation(3).unwrap();
    // Then
    let aapl_expected = panel
        .get_series("AAPL")
        .unwrap()
        .rolling_standard_deviation(3)
        .unwrap();
    assert_f64_vecs_eq(
        &result.get_series("AAPL").unwrap().values,
        &aapl_expected.values,
    );
    let msft_expected = panel
        .get_series("MSFT")
        .unwrap()
        .rolling_standard_deviation(3)
        .unwrap();
    assert_f64_vecs_eq(
        &result.get_series("MSFT").unwrap().values,
        &msft_expected.values,
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_true_range__then_matches_time_series_true_range() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.true_range().unwrap();
    // Then
    let aapl_expected = panel.get_series("AAPL").unwrap().true_range().unwrap();
    assert_f64_vecs_eq(
        &result.get_series("AAPL").unwrap().values,
        &aapl_expected.values,
    );
    let msft_expected = panel.get_series("MSFT").unwrap().true_range().unwrap();
    assert_f64_vecs_eq(
        &result.get_series("MSFT").unwrap().values,
        &msft_expected.values,
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_average_true_range__then_matches_time_series_average_true_range() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.average_true_range(3).unwrap();
    // Then
    let aapl_expected = panel
        .get_series("AAPL")
        .unwrap()
        .average_true_range(3)
        .unwrap();
    assert_f64_vecs_eq(
        &result.get_series("AAPL").unwrap().values,
        &aapl_expected.values,
    );
    let msft_expected = panel
        .get_series("MSFT")
        .unwrap()
        .average_true_range(3)
        .unwrap();
    assert_f64_vecs_eq(
        &result.get_series("MSFT").unwrap().values,
        &msft_expected.values,
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_bollinger_bands__then_matches_time_series_bollinger_bands() {
    // Given
    let panel = sample_panel();
    // When
    let (upper, mid, lower) = panel.bollinger_bands(3, 2.0).unwrap();
    // Then — AAPL
    let (ts_upper, ts_mid, ts_lower) = panel
        .get_series("AAPL")
        .unwrap()
        .bollinger_bands(3, 2.0)
        .unwrap();
    assert_f64_vecs_eq(&upper.get_series("AAPL").unwrap().values, &ts_upper.values);
    assert_f64_vecs_eq(&mid.get_series("AAPL").unwrap().values, &ts_mid.values);
    assert_f64_vecs_eq(&lower.get_series("AAPL").unwrap().values, &ts_lower.values);
    // Then — MSFT
    let (ts_upper_m, ts_mid_m, ts_lower_m) = panel
        .get_series("MSFT")
        .unwrap()
        .bollinger_bands(3, 2.0)
        .unwrap();
    assert_f64_vecs_eq(
        &upper.get_series("MSFT").unwrap().values,
        &ts_upper_m.values,
    );
    assert_f64_vecs_eq(&mid.get_series("MSFT").unwrap().values, &ts_mid_m.values);
    assert_f64_vecs_eq(
        &lower.get_series("MSFT").unwrap().values,
        &ts_lower_m.values,
    );
}

// ---------------------------------------------------------------------------
// Autocorrelation
// ---------------------------------------------------------------------------

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_autocorrelation_function__then_matches_time_series_acf() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.autocorrelation_function(1).unwrap();
    // Then
    assert_eq!(
        result["AAPL"],
        panel
            .get_series("AAPL")
            .unwrap()
            .autocorrelation_function(1)
            .unwrap()
    );
    assert_eq!(
        result["MSFT"],
        panel
            .get_series("MSFT")
            .unwrap()
            .autocorrelation_function(1)
            .unwrap()
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_partial_autocorrelation_function__then_matches_time_series_pacf() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.partial_autocorrelation_function(2).unwrap();
    // Then
    assert_eq!(
        result["AAPL"],
        panel
            .get_series("AAPL")
            .unwrap()
            .partial_autocorrelation_function(2)
            .unwrap()
    );
    assert_eq!(
        result["MSFT"],
        panel
            .get_series("MSFT")
            .unwrap()
            .partial_autocorrelation_function(2)
            .unwrap()
    );
}

// ---------------------------------------------------------------------------
// Stationarity
// ---------------------------------------------------------------------------

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_stationary_dickey_fuller_statistics__then_matches_time_series_df_stat() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.stationary_dickey_fuller_statistics().unwrap();
    // Then
    assert_eq!(
        result["AAPL"],
        panel
            .get_series("AAPL")
            .unwrap()
            .stationary_dickey_fuller_statistics()
            .unwrap()
    );
    assert_eq!(
        result["MSFT"],
        panel
            .get_series("MSFT")
            .unwrap()
            .stationary_dickey_fuller_statistics()
            .unwrap()
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_stationary_dickey_fuller_test__then_matches_time_series_df_test() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.stationary_dickey_fuller_test(0.05).unwrap();
    // Then
    assert_eq!(
        result["AAPL"],
        panel
            .get_series("AAPL")
            .unwrap()
            .stationary_dickey_fuller_test(0.05)
            .unwrap()
    );
    assert_eq!(
        result["MSFT"],
        panel
            .get_series("MSFT")
            .unwrap()
            .stationary_dickey_fuller_test(0.05)
            .unwrap()
    );
}

// ---------------------------------------------------------------------------
// Distribution analysis
// ---------------------------------------------------------------------------

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_skewness__then_matches_time_series_skewness() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.skewness().unwrap();
    // Then
    assert_eq!(
        result["AAPL"],
        panel.get_series("AAPL").unwrap().skewness().unwrap()
    );
    assert_eq!(
        result["MSFT"],
        panel.get_series("MSFT").unwrap().skewness().unwrap()
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_excess_kurtosis__then_matches_time_series_excess_kurtosis() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.excess_kurtosis().unwrap();
    // Then
    assert_eq!(
        result["AAPL"],
        panel.get_series("AAPL").unwrap().excess_kurtosis().unwrap()
    );
    assert_eq!(
        result["MSFT"],
        panel.get_series("MSFT").unwrap().excess_kurtosis().unwrap()
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_jacque_bera_statistics__then_matches_time_series_jb_stat() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.jacque_bera_statistics().unwrap();
    // Then
    assert_eq!(
        result["AAPL"],
        panel
            .get_series("AAPL")
            .unwrap()
            .jacque_bera_statistics()
            .unwrap()
    );
    assert_eq!(
        result["MSFT"],
        panel
            .get_series("MSFT")
            .unwrap()
            .jacque_bera_statistics()
            .unwrap()
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_panel__when_jacque_bera_test__then_matches_time_series_jb_test() {
    // Given
    let panel = sample_panel();
    // When
    let result = panel.jacque_bera_test(0.05).unwrap();
    // Then
    assert_eq!(
        result["AAPL"],
        panel
            .get_series("AAPL")
            .unwrap()
            .jacque_bera_test(0.05)
            .unwrap()
    );
    assert_eq!(
        result["MSFT"],
        panel
            .get_series("MSFT")
            .unwrap()
            .jacque_bera_test(0.05)
            .unwrap()
    );
}
