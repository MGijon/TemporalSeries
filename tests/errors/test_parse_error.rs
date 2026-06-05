use std::error::Error;
use std::fs;

use temporalseries::errors::TemporalSeriesError;
use temporalseries::io::read_csv;

#[test]
#[allow(non_snake_case)]
fn test__given_malformed_csv__when_read_csv__then_raises_ParseError() {
    // Given — write to the system temp dir so the path always exists in CI
    let path = std::env::temp_dir().join("temporalseries_malformed_test.csv");
    fs::write(&path, "index,value\nnot_a_number,1.0\n").unwrap();

    // When
    let result = read_csv(path.to_str().unwrap());
    fs::remove_file(&path).unwrap();

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::ParseError(_))));
}

#[test]
#[allow(non_snake_case)]
fn test__given_ParseError__when_display__then_message_contains_parse_error_prefix() {
    // Given
    let err = TemporalSeriesError::ParseError("line 2: invalid index".to_string());

    // When
    let msg = err.to_string();

    // Then
    assert!(
        msg.contains("parse error"),
        "expected 'parse error' prefix in message: {msg}"
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_ParseError__when_display__then_message_contains_inner_text() {
    // Given
    let inner = "line 2: invalid index".to_string();
    let err = TemporalSeriesError::ParseError(inner.clone());

    // When
    let msg = err.to_string();

    // Then
    assert!(
        msg.contains(&inner),
        "expected inner message in display: {msg}"
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_ParseError__when_check_source__then_returns_none() {
    // Given
    let err = TemporalSeriesError::ParseError("some parse error".to_string());

    // Then
    assert!(err.source().is_none());
}

#[test]
#[allow(non_snake_case)]
fn test__given_ParseError__when_debug__then_does_not_panic() {
    // Given
    let err = TemporalSeriesError::ParseError("some parse error".to_string());

    // Then
    let _ = format!("{err:?}");
}
