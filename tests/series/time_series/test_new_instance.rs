use temporalseries::errors::TemporalSeriesError;
use temporalseries::series::TimeSeries;

#[test]
#[allow(non_snake_case)]
fn test__given_different_lenght_vectors__when_create_new_ts__then_raise_LengthMismatch_error() {
    // Given
    let index: Vec<i64> = vec![1, 2, 3];
    let values: Vec<f64> = vec![10.0, 20.0];

    // When
    let result: Result<TimeSeries, TemporalSeriesError> = TimeSeries::new(index, values);

    // Then
    assert!(matches!(
        result,
        Err(TemporalSeriesError::LengthMismatch { .. })
    ));
}
