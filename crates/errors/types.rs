use std::fmt;

// TODO: document this errors!
#[derive(Debug)]
pub enum TemporalSeriesError {
    LengthMismatch { index_len: usize, values_len: usize },

    EmptySeries,

    InvalidWindow { window: usize, series_len: usize },

    IoError(std::io::Error),

    ParseError(String),
}

impl fmt::Display for TemporalSeriesError {
    // TODO: document this methods!
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

            TemporalSeriesError::IoError(e) => write!(f, "IO error: {}", e),

            TemporalSeriesError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl From<std::io::Error> for TemporalSeriesError {
    fn from(e: std::io::Error) -> Self {
        TemporalSeriesError::IoError(e)
    }
}

impl std::error::Error for TemporalSeriesError {}
