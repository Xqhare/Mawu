use std::{fmt, result};

pub mod csv_error;
pub mod json_error;

#[derive(Debug)]
/// MawuError wraps all errors that can occur in Mawu.
/// These are mainly `IoError`'s and parsing errors.
pub enum MawuError {
    /// A wrapper for `std::io::Error` only used for file handling
    IoError(std::io::Error),
    /// A wrapper for `csv::Error` containing all errors for CSV
    CsvError(csv_error::CsvError),
    /// A wrapper for `json::Error` containing all errors for JSON
    JsonError(json_error::JsonError),
    /// A wrapper for internal errors. If you ever see this, please file an issue.
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
/// Internal errors, If you ever see this, please file an issue.
pub enum MawuInternalError {
    /// Fail-safe if unable to lock the master mutex of the character queue
    UnableToLockMasterMutex,
    /// Fail-safe if Mawu encountered a String with no chars
    StringWithNoChars(String),
    /// Fail-save if unable to unescape unicode
    UnableToUnescapeUnicode(String),
}

impl fmt::Display for MawuInternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MawuInternalError::UnableToLockMasterMutex => write!(f, "Unable to lock mutex"),
            MawuInternalError::StringWithNoChars(ref s) => write!(f, "String with no chars: {}", s),
            MawuInternalError::UnableToUnescapeUnicode(ref s) => {
                write!(f, "Unable to unescape unicode: {}", s)
            }
        }
    }
}
