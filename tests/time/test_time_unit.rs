#![cfg(feature = "chrono")]

use chrono::{TimeZone, Utc};
use temporalseries::time::TimeUnit;

// — from_datetime —

#[test]
#[allow(non_snake_case)]
fn test__given_datetime__when_from_datetime_seconds__then_returns_unix_seconds() {
    // Given
    let dt = Utc.timestamp_opt(1_000, 0).unwrap();
    // When / Then
    assert_eq!(TimeUnit::Seconds.from_datetime(dt), 1_000);
}

#[test]
#[allow(non_snake_case)]
fn test__given_datetime__when_from_datetime_milliseconds__then_returns_unix_millis() {
    // Given
    let dt = Utc.timestamp_opt(1, 0).unwrap(); // 1 second = 1 000 ms
    // When / Then
    assert_eq!(TimeUnit::Milliseconds.from_datetime(dt), 1_000);
}

#[test]
#[allow(non_snake_case)]
fn test__given_datetime__when_from_datetime_microseconds__then_returns_unix_micros() {
    // Given
    let dt = Utc.timestamp_opt(1, 0).unwrap(); // 1 second = 1 000 000 µs
    // When / Then
    assert_eq!(TimeUnit::Microseconds.from_datetime(dt), 1_000_000);
}

#[test]
#[allow(non_snake_case)]
fn test__given_datetime__when_from_datetime_nanoseconds__then_returns_unix_nanos() {
    // Given
    let dt = Utc.timestamp_opt(1, 0).unwrap(); // 1 second = 1 000 000 000 ns
    // When / Then
    assert_eq!(TimeUnit::Nanoseconds.from_datetime(dt), 1_000_000_000);
}

// — to_datetime —

#[test]
#[allow(non_snake_case)]
fn test__given_seconds_timestamp__when_to_datetime__then_returns_correct_datetime() {
    // Given / When
    let dt = TimeUnit::Seconds.to_datetime(1_000).unwrap();
    // Then
    assert_eq!(dt.timestamp(), 1_000);
}

#[test]
#[allow(non_snake_case)]
fn test__given_milliseconds_timestamp__when_to_datetime__then_returns_correct_datetime() {
    // Given / When
    let dt = TimeUnit::Milliseconds.to_datetime(1_500).unwrap(); // 1.5 seconds
    // Then
    assert_eq!(dt.timestamp(), 1);
    assert_eq!(dt.timestamp_millis(), 1_500);
}

#[test]
#[allow(non_snake_case)]
fn test__given_microseconds_timestamp__when_to_datetime__then_returns_correct_datetime() {
    // Given / When
    let dt = TimeUnit::Microseconds.to_datetime(1_500_000).unwrap(); // 1.5 seconds
    // Then
    assert_eq!(dt.timestamp(), 1);
    assert_eq!(dt.timestamp_micros(), 1_500_000);
}

#[test]
#[allow(non_snake_case)]
fn test__given_nanoseconds_timestamp__when_to_datetime__then_returns_correct_datetime() {
    // Given / When
    let dt = TimeUnit::Nanoseconds.to_datetime(1_500_000_000).unwrap(); // 1.5 seconds
    // Then
    assert_eq!(dt.timestamp(), 1);
}

// — round-trip —

#[test]
#[allow(non_snake_case)]
fn test__given_datetime__when_round_trip_seconds__then_recovers_original() {
    // Given
    let dt = Utc.timestamp_opt(42, 0).unwrap();
    // When
    let ts = TimeUnit::Seconds.from_datetime(dt);
    let recovered = TimeUnit::Seconds.to_datetime(ts).unwrap();
    // Then
    assert_eq!(dt, recovered);
}

#[test]
#[allow(non_snake_case)]
fn test__given_datetime__when_round_trip_milliseconds__then_recovers_original() {
    // Given
    let dt = Utc.timestamp_millis_opt(123_456).unwrap();
    // When
    let ts = TimeUnit::Milliseconds.from_datetime(dt);
    let recovered = TimeUnit::Milliseconds.to_datetime(ts).unwrap();
    // Then
    assert_eq!(dt, recovered);
}

// — negative timestamps (before Unix epoch) —

#[test]
#[allow(non_snake_case)]
fn test__given_negative_milliseconds_timestamp__when_to_datetime__then_returns_correct_datetime() {
    // Given: -500 ms = 0.5 s before epoch
    let dt = TimeUnit::Milliseconds.to_datetime(-500).unwrap();
    // Then
    assert_eq!(dt.timestamp_millis(), -500);
}
