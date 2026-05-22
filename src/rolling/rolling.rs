use crate::series::TimeSeries;

pub struct RollingSeries<'a> {
    series: &'a TimeSeries,
    window: usize,
}

impl<'a> RollingSeries<'a> {
    pub fn new(series: &'a TimeSeries, window: usize) -> Self {
        Self { series, window }
    }

    pub fn mean(&self) -> TimeSeries {
        let n = self.series.len();
        let mut result = vec![f64::NAN; n];

        for i in self.window - 1..n {
            let slice = &self.series.values[i + 1 - self.window..=i];
            let mean = slice.iter().sum::<f64>() / self.window as f64;
            result[i] = mean;
        }

        TimeSeries::new(self.series.index.clone(), result)
    }
}
