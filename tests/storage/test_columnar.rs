use temporalseries::storage::{ColumnarBackend, StorageBackend};

#[test]
#[allow(non_snake_case)]
fn test__given_values__when_new__then_len_is_correct() {
    // Given & When
    let backend: ColumnarBackend<f64> = ColumnarBackend::new(vec![1.0_f64, 2.0, 3.0]);
    // Then
    assert_eq!(backend.len(), 3);
}

#[test]
#[allow(non_snake_case)]
fn test__given_empty_backend__when_is_empty__then_returns_true() {
    // Given & When
    let backend: ColumnarBackend<f64> = ColumnarBackend::<f64>::new(vec![]);
    // Then
    assert!(backend.is_empty());
}

#[test]
#[allow(non_snake_case)]
fn test__given_values__when_get_valid_index__then_returns_value() {
    // Given & When
    let backend: ColumnarBackend<f64> = ColumnarBackend::new(vec![10.0_f64, 20.0, 30.0]);
    // Then
    assert_eq!(backend.get(1), Some(&20.0));
}

#[test]
#[allow(non_snake_case)]
fn test__given_values__when_get_out_of_bounds__then_returns_none() {
    // Given & When
    let backend: ColumnarBackend<f64> = ColumnarBackend::new(vec![1.0_f64]);
    // Then
    assert!(backend.get(99).is_none());
}

#[test]
#[allow(non_snake_case)]
fn test__given_backend__when_push__then_len_increases() {
    // Given & When
    let mut backend: ColumnarBackend<f64> = ColumnarBackend::new(vec![1.0_f64, 2.0]);
    backend.push(3.0);
    // Then
    assert_eq!(backend.len(), 3);
    assert_eq!(backend.get(2), Some(&3.0));
}

#[test]
#[allow(non_snake_case)]
fn test__given_backend__when_slice__then_returns_correct_subset() {
    let backend: ColumnarBackend<f64> = ColumnarBackend::new(vec![10.0_f64, 20.0, 30.0, 40.0]);
    let sliced: ColumnarBackend<f64> = backend.slice(1..3);
    assert_eq!(sliced.len(), 2);
    assert_eq!(sliced.get(0), Some(&20.0));
    assert_eq!(sliced.get(1), Some(&30.0));
}

#[test]
#[allow(non_snake_case)]
fn test__given_backend__when_iter__then_yields_values_in_order() {
    let backend: ColumnarBackend<f64> = ColumnarBackend::new(vec![1.0_f64, 2.0, 3.0]);
    let collected: Vec<&f64> = backend.iter().collect();
    assert_eq!(collected, vec![&1.0, &2.0, &3.0]);
}
