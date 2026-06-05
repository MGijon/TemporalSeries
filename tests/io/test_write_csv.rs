use temporalseries::errors::TemporalSeriesError;
use temporalseries::io::write_csv;
use temporalseries::series::TimeSeries;

#[test]
#[allow(non_snake_case)]
fn test__given_valid_data_and_file_path__when_write_csv__then_writes_correctly() {
    // Given
    let file_path: &str = "test_output.csv";
    let data: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![100.0, 110.0, 121.0]).unwrap();
    // When
    let result: Result<(), TemporalSeriesError> = write_csv(&data, file_path);
    // Then
    assert!(result.is_ok());
}

#[test]
#[allow(non_snake_case)]
fn test__given_invalid_file_path__when_write_csv__then_returns_error() {
    // Given
    let file_path: &str = "/invalid_path/test_output.csv";
    let data: TimeSeries = TimeSeries::new(vec![1, 2, 3], vec![100.0, 110.0, 121.0]).unwrap();
    // When
    let result: Result<(), TemporalSeriesError> = write_csv(&data, file_path);
    // Then
    assert!(result.is_err());
}
