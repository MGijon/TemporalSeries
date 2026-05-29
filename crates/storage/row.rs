use std::ops::Range;

use super::StorageBackend;

pub struct RowRecord<T> {
    pub timestamp: i64,
    pub value: T,
}

pub struct RowBackend<T> {
    rows: Vec<RowRecord<T>>,
}

impl<T> RowBackend<T> {
    pub fn new(rows: Vec<RowRecord<T>>) -> Self {
        Self { rows }
    }
}

impl<T: Clone + Send + Sync> StorageBackend<T> for RowBackend<T> {
    fn len(&self) -> usize {
        self.rows.len()
    }

    fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    fn get(&self, idx: usize) -> Option<&T> {
        self.rows.get(idx).map(|r| &r.value)
    }

    fn push(&mut self, value: T) {
        let timestamp = self.rows.len() as i64;
        self.rows.push(RowRecord { timestamp, value });
    }

    fn slice(&self, range: Range<usize>) -> Self {
        Self {
            rows: self.rows[range]
                .iter()
                .map(|r| RowRecord {
                    timestamp: r.timestamp,
                    value: r.value.clone(),
                })
                .collect(),
        }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.rows.iter().map(|r| &r.value))
    }
}
