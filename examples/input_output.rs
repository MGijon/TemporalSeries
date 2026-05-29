use temporalseries::io::{read_csv, write_csv};
use temporalseries::series::TimeSeries;

fn main() {
    let time_serie: TimeSeries = read_csv("examples/input.csv").unwrap();
    println!("values: {:?}", time_serie.values);

    write_csv(&time_serie, "examples/output.csv").unwrap();

    let read_back: TimeSeries = read_csv("examples/output.csv").unwrap();
    println!("round-trip: {:?}", read_back.values);
}
