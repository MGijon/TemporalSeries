use std::error::Error;

use temporalseries::errors::TemporalSeriesError;

#[test]
#[allow(non_snake_case)]
fn test__given_EmptySeries__when_display__then_message_is_correct() {
    // Given
    let err = TemporalSeriesError::EmptySeries;

    // When
    let msg = err.to_string();

    // Then
    assert_eq!(msg, "series is empty");
}

#[test]
#[allow(non_snake_case)]
fn test__given_EmptySeries__when_check_source__then_returns_none() {
    // Given
    let err = TemporalSeriesError::EmptySeries;

    // Then
    assert!(err.source().is_none());
}

#[test]
#[allow(non_snake_case)]
fn test__given_EmptySeries__when_debug__then_does_not_panic() {
    // Given
    let err = TemporalSeriesError::EmptySeries;

    // Then
    let _ = format!("{err:?}");
}
