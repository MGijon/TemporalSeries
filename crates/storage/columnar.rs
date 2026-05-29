use std::ops::Range;

use super::StorageBackend;

pub struct ColumnarBackend<T> {
    data: Vec<T>,
}

impl<T> ColumnarBackend<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T: Clone + Send + Sync> StorageBackend<T> for ColumnarBackend<T> {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn get(&self, idx: usize) -> Option<&T> {
        self.data.get(idx)
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    fn slice(&self, range: Range<usize>) -> Self {
        Self {
            data: self.data[range].to_vec(),
        }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.data.iter())
    }
}
