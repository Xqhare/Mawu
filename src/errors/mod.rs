use std::{fmt, result};

pub mod csv_error;

pub enum MawuError {
    IoError(std::io::Error),
    CsvError(csv_error::CsvError),
}

pub type Result<T> = result::Result<T, MawuError>;

impl fmt::Display for MawuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MawuError::IoError(ref e) => e.fmt(f),
            MawuError::CsvError(ref e) => e.fmt(f),
        }
    }
    
}
