use temporalseries::series::TimeSeries;

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn test__given_different_lenght_vectors__when_create_new_ts__then_panics() {
    // Given
    let index = vec![1, 2, 3];
    let values = vec![10.0, 20.0];

    // When & Then
    TimeSeries::new(index, values);
}
