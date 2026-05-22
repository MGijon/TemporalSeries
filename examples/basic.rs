use temporalseries::series::TimeSeries;

fn main() {
    let index = vec![1, 2, 3, 4, 5];
    let values = vec![100.0, 101.0, 102.0, 103.0, 104.0];

    let time_serie = TimeSeries::new(index, values);

    let returns = time_serie.pct_change();
    let momentum = returns.rolling(2).mean();

    println!("{:?}", momentum.values);
}
