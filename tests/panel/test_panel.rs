use temporalseries::errors::TemporalSeriesError;
use temporalseries::panel::Panel;
use temporalseries::series::TimeSeries;

fn sample_panel() -> Panel {
    Panel::new(
        vec![1, 2, 3],
        vec!["AAPL".into(), "MSFT".into()],
        vec![vec![150.0, 152.0, 149.0], vec![300.0, 305.0, 298.0]],
    )
    .unwrap()
}

#[test]
#[allow(non_snake_case)]
fn test__given_valid_inputs__when_new__then_panel_is_created() {
    // Given & When
    let panel: Panel = sample_panel();
    // Then
    assert_eq!(panel.shape(), (3, 2));
}

#[test]
#[allow(non_snake_case)]
fn test__given_mismatched_symbols_and_values__when_new__then_returns_error() {
    let result: Result<Panel, TemporalSeriesError> = Panel::new(
        vec![1, 2, 3],
        vec!["AAPL".into()],
        vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
    );
    assert!(matches!(
        result,
        Err(TemporalSeriesError::LengthMismatch { .. })
    ));
}

#[test]
#[allow(non_snake_case)]
fn test__given_series_shorter_than_index__when_new__then_returns_error() {
    let result: Result<Panel, TemporalSeriesError> =
        Panel::new(vec![1, 2, 3], vec!["AAPL".into()], vec![vec![1.0, 2.0]]);
    assert!(matches!(
        result,
        Err(TemporalSeriesError::LengthMismatch { .. })
    ));
}

#[test]
#[allow(non_snake_case)]
fn test__given_valid_panel__when_len__then_returns_number_of_timestamps() {
    let panel: Panel = sample_panel();
    assert_eq!(panel.len(), 3);
}

#[test]
#[allow(non_snake_case)]
fn test__given_valid_panel__when_n_series__then_returns_number_of_columns() {
    let panel: Panel = sample_panel();
    assert_eq!(panel.n_series(), 2);
}

#[test]
#[allow(non_snake_case)]
fn test__given_valid_panel__when_symbols__then_returns_symbol_names() {
    let panel: Panel = sample_panel();
    assert_eq!(panel.symbols(), &["AAPL", "MSFT"]);
}

#[test]
#[allow(non_snake_case)]
fn test__given_valid_panel__when_get_series_known_symbol__then_returns_series() {
    let panel: Panel = sample_panel();
    let ts: TimeSeries = panel.get_series("AAPL").unwrap();
    assert_eq!(ts.values, vec![150.0, 152.0, 149.0]);
}

#[test]
#[allow(non_snake_case)]
fn test__given_valid_panel__when_get_series_unknown_symbol__then_returns_none() {
    let panel: Panel = sample_panel();
    assert!(panel.get_series("GOOG").is_none());
}

#[test]
#[allow(non_snake_case)]
fn test__given_empty_panel__when_is_empty__then_returns_true() {
    let panel: Panel = Panel::new(vec![], vec![], vec![]).unwrap();
    assert!(panel.is_empty());
}
