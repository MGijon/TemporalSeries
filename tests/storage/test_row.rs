use temporalseries::storage::{RowBackend, RowRecord, StorageBackend};

fn sample_backend() -> RowBackend<f64> {
    RowBackend::new(vec![
        RowRecord {
            timestamp: 1,
            value: 10.0,
        },
        RowRecord {
            timestamp: 2,
            value: 20.0,
        },
        RowRecord {
            timestamp: 3,
            value: 30.0,
        },
    ])
}

#[test]
#[allow(non_snake_case)]
fn test__given_records__when_new__then_len_is_correct() {
    // Given & When
    let backend: RowBackend<f64> = sample_backend();
    // Then
    assert_eq!(backend.len(), 3);
}

#[test]
#[allow(non_snake_case)]
fn test__given_empty_backend__when_is_empty__then_returns_true() {
    // Given & When
    let backend: RowBackend<f64> = RowBackend::<f64>::new(vec![]);
    // Then
    assert!(backend.is_empty());
}

#[test]
#[allow(non_snake_case)]
fn test__given_records__when_get_valid_index__then_returns_value() {
    // Given & When
    let backend: RowBackend<f64> = sample_backend();
    // Then
    assert_eq!(backend.get(1), Some(&20.0));
}

#[test]
#[allow(non_snake_case)]
fn test__given_records__when_get_out_of_bounds__then_returns_none() {
    // Given & When
    let backend: RowBackend<f64> = sample_backend();
    // Then
    assert!(backend.get(99).is_none());
}

#[test]
#[allow(non_snake_case)]
fn test__given_backend__when_push__then_len_increases_and_value_is_accessible() {
    // Given
    let mut backend: RowBackend<f64> = sample_backend();
    // When
    backend.push(40.0);
    // Then
    assert_eq!(backend.len(), 4);
    assert_eq!(backend.get(3), Some(&40.0));
}

#[test]
#[allow(non_snake_case)]
fn test__given_backend__when_slice__then_returns_correct_subset() {
    // Given & When
    let backend: RowBackend<f64> = sample_backend();
    let sliced: RowBackend<f64> = backend.slice(1..3);
    // Then
    assert_eq!(sliced.len(), 2);
    assert_eq!(sliced.get(0), Some(&20.0));
    assert_eq!(sliced.get(1), Some(&30.0));
}

#[test]
#[allow(non_snake_case)]
fn test__given_backend__when_iter__then_yields_values_in_order() {
    // Given & When
    let backend: RowBackend<f64> = sample_backend();
    let collected: Vec<&f64> = backend.iter().collect();
    assert_eq!(collected, vec![&10.0, &20.0, &30.0]);
}
