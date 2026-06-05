use std::error::Error;

use temporalseries::errors::TemporalSeriesError;

#[test]
#[allow(non_snake_case)]
fn test__given_InvalidWindow__when_display__then_message_contains_window_and_series_len() {
    // Given
    let err = TemporalSeriesError::InvalidWindow {
        window: 10,
        series_len: 3,
    };

    // When
    let msg = err.to_string();

    // Then
    assert!(msg.contains("10"), "expected window 10 in message: {msg}");
    assert!(msg.contains('3'), "expected series_len 3 in message: {msg}");
}

#[test]
#[allow(non_snake_case)]
fn test__given_InvalidWindow__when_check_source__then_returns_none() {
    // Given
    let err = TemporalSeriesError::InvalidWindow {
        window: 10,
        series_len: 3,
    };

    // Then
    assert!(err.source().is_none());
}

#[test]
#[allow(non_snake_case)]
fn test__given_InvalidWindow__when_debug__then_does_not_panic() {
    // Given
    let err = TemporalSeriesError::InvalidWindow {
        window: 10,
        series_len: 3,
    };

    // Then
    let _ = format!("{err:?}");
}
