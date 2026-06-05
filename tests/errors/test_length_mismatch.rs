use std::error::Error;

use temporalseries::errors::TemporalSeriesError;
use temporalseries::series::TimeSeries;

#[test]
#[allow(non_snake_case)]
fn test__given_mismatched_lengths__when_create_time_series__then_raises_LengthMismatch() {
    // Given
    let index = vec![1_i64, 2, 3];
    let values = vec![10.0_f64, 20.0];

    // When
    let result = TimeSeries::new(index, values);

    // Then
    assert!(matches!(
        result,
        Err(TemporalSeriesError::LengthMismatch { .. })
    ));
}

#[test]
#[allow(non_snake_case)]
fn test__given_LengthMismatch__when_display__then_message_contains_both_lengths() {
    // Given
    let err = TemporalSeriesError::LengthMismatch {
        index_len: 3,
        values_len: 2,
    };

    // When
    let msg = err.to_string();

    // Then
    assert!(msg.contains('3'), "expected index_len 3 in message: {msg}");
    assert!(msg.contains('2'), "expected values_len 2 in message: {msg}");
}

#[test]
#[allow(non_snake_case)]
fn test__given_LengthMismatch__when_check_source__then_returns_none() {
    // Given
    let err = TemporalSeriesError::LengthMismatch {
        index_len: 3,
        values_len: 2,
    };

    // Then
    assert!(err.source().is_none());
}

#[test]
#[allow(non_snake_case)]
fn test__given_LengthMismatch__when_debug__then_does_not_panic() {
    // Given
    let err = TemporalSeriesError::LengthMismatch {
        index_len: 3,
        values_len: 2,
    };

    // Then
    let _ = format!("{err:?}");
}
