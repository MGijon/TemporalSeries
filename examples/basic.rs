use temporalseries::series::TimeSeries;

fn main() {
    let index: Vec<i64> = vec![1, 2, 3, 4, 5];
    let values: Vec<f64> = vec![100.0, 101.0, 102.0, 103.0, 104.0];

    let time_serie: TimeSeries = TimeSeries::new(index, values).unwrap();

    let returns: TimeSeries = time_serie.pct_change().unwrap();
    let momentum: TimeSeries = returns.rolling(2).mean().unwrap();

    println!("{:?}", momentum.values);
}
