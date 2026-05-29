use crate::errors::TemporalSeriesError;
use crate::series::TimeSeries;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/// Reads a CSV file into a [`TimeSeries`].
///
/// The file must have a header row followed by one row per observation.
/// Each row must contain exactly two comma-separated fields: `index` (`i64`)
/// and `value` (`f64`).
///
/// ```text
/// index,value
/// 1,100.0
/// 2,101.5
/// 3,98.0
/// ```
///
/// # Errors
///
/// - [`TemporalSeriesError::IoError`] if the file cannot be opened or read.
/// - [`TemporalSeriesError::ParseError`] if a row is malformed or a field
///   cannot be parsed.
pub fn read_csv(path: &str) -> Result<TimeSeries, TemporalSeriesError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // skip header
    lines.next();

    let mut index = Vec::new();
    let mut values = Vec::new();

    for (line_no, line) in lines.enumerate() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.splitn(2, ',');

        let idx = parts
            .next()
            .ok_or_else(|| {
                TemporalSeriesError::ParseError(format!(
                    "line {}: missing index field",
                    line_no + 2
                ))
            })?
            .trim()
            .parse::<i64>()
            .map_err(|e| {
                TemporalSeriesError::ParseError(format!(
                    "line {}: invalid index — {}",
                    line_no + 2,
                    e
                ))
            })?;

        let val = parts
            .next()
            .ok_or_else(|| {
                TemporalSeriesError::ParseError(format!(
                    "line {}: missing value field",
                    line_no + 2
                ))
            })?
            .trim()
            .parse::<f64>()
            .map_err(|e| {
                TemporalSeriesError::ParseError(format!(
                    "line {}: invalid value — {}",
                    line_no + 2,
                    e
                ))
            })?;

        index.push(idx);
        values.push(val);
    }

    TimeSeries::new(index, values)
}

/// Writes a [`TimeSeries`] to a CSV file.
///
/// The output has a header row (`index,value`) followed by one row per
/// observation.
///
/// ```text
/// index,value
/// 1,100.0
/// 2,101.5
/// 3,98.0
/// ```
///
/// # Errors
///
/// Returns [`TemporalSeriesError::IoError`] if the file cannot be created
/// or written.
pub fn write_csv(ts: &TimeSeries, path: &str) -> Result<(), TemporalSeriesError> {
    let mut file = File::create(path)?;

    writeln!(file, "index,value")?;

    for (idx, val) in ts.index.iter().zip(ts.values.iter()) {
        writeln!(file, "{},{}", idx, val)?;
    }

    Ok(())
}
