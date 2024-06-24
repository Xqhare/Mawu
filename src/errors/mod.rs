use std::{fmt, result};

pub mod csv_error;
pub mod json_error;

#[derive(Debug)]
pub enum MawuError {
    IoError(std::io::Error),
    CsvError(csv_error::CsvError),
    JsonError(json_error::JsonError),
    InternalError(MawuInternalError),
}

pub type Result<T> = result::Result<T, MawuError>;

impl fmt::Display for MawuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MawuError::IoError(ref e) => e.fmt(f),
            MawuError::CsvError(ref e) => e.fmt(f),
            MawuError::JsonError(ref e) => e.fmt(f),
            MawuError::InternalError(ref e) => e.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum MawuInternalError {
    UnableToLockMasterMutex,
    StringWithNoChars(String),
}

impl fmt::Display for MawuInternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MawuInternalError::UnableToLockMasterMutex => write!(f, "Unable to lock mutex"),
            MawuInternalError::StringWithNoChars(ref s) => write!(f, "String with no chars: {}", s),
        }
    }
}
