use std::ops::Range;

pub trait StorageBackend<T>: Send + Sync {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn get(&self, idx: usize) -> Option<&T>;
    fn push(&mut self, value: T);
    fn slice(&self, range: Range<usize>) -> Self;
    fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_>;
}
