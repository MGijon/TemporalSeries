use std::error::Error;
use std::io;

use temporalseries::errors::TemporalSeriesError;
use temporalseries::io::read_csv;

#[test]
#[allow(non_snake_case)]
fn test__given_non_existent_file__when_read_csv__then_raises_IoError() {
    // Given
    let path = "non_existent_file.csv";

    // When
    let result = read_csv(path);

    // Then
    assert!(matches!(result, Err(TemporalSeriesError::IoError(_))));
}

#[test]
#[allow(non_snake_case)]
fn test__given_IoError__when_display__then_message_contains_io_error_prefix() {
    // Given
    let inner = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err = TemporalSeriesError::IoError(inner);

    // When
    let msg = err.to_string();

    // Then
    assert!(
        msg.contains("IO error"),
        "expected 'IO error' prefix in message: {msg}"
    );
}

#[test]
#[allow(non_snake_case)]
fn test__given_IoError__when_check_source__then_returns_some() {
    // Given
    let inner = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err = TemporalSeriesError::IoError(inner);

    // Then
    assert!(err.source().is_some());
}

#[test]
#[allow(non_snake_case)]
fn test__given_std_io_error__when_convert_via_from__then_wraps_as_IoError() {
    // Given
    let inner = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");

    // When
    let err = TemporalSeriesError::from(inner);

    // Then
    assert!(matches!(err, TemporalSeriesError::IoError(_)));
}

#[test]
#[allow(non_snake_case)]
fn test__given_IoError__when_debug__then_does_not_panic() {
    // Given
    let inner = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err = TemporalSeriesError::IoError(inner);

    // Then
    let _ = format!("{err:?}");
}
