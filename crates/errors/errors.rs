use std::fmt;

#[derive(Debug)]
pub enum TemporalSeriesError {
    LengthMismatch { index_len: usize, values_len: usize },

    EmptySeries,

    InvalidWindow { window: usize, series_len: usize },
}

impl fmt::Display for TemporalSeriesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemporalSeriesError::LengthMismatch {
                index_len,
                values_len,
            } => write!(
                f,
                "Length mismatch: index has length {}, values has length {}",
                index_len, values_len
            ),

            TemporalSeriesError::EmptySeries => {
                write!(f, "TimeSeries is empty")
            }

            TemporalSeriesError::InvalidWindow { window, series_len } => write!(
                f,
                "Invalid rolling window {} for series lenght {}",
                window, series_len
            ),
        }
    }
}

impl std::error::Error for TemporalSeriesError {}
