use temporalseries::errors::TemporalSeriesError;
use temporalseries::io::read_csv;
use temporalseries::series::TimeSeries;

#[test]
#[allow(non_snake_case)]
fn test__given_valid_csv_file__when_read_csv__then_reads_correctly() {
    // Given
    let file_path: &str = "examples/input.csv";
    // When
    let result: Result<TimeSeries, TemporalSeriesError> = read_csv(file_path);
    // Then
    assert!(result.is_ok());
}

#[test]
#[allow(non_snake_case)]
fn test__given_non_existent_csv_file__when_read_csv__then_returns_error() {
    // Given
    let file_path = "non_existent_file.csv";
    // When
    let result: Result<TimeSeries, TemporalSeriesError> = read_csv(file_path);
    // Then
    assert!(result.is_err());
}
