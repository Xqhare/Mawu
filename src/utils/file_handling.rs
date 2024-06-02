use std::{fs::read_to_string, path::Path};

use crate::errors::MawuError;

/// This function reads the contents of a file and returns it as a single String.
/// It only accepts valid UTF-8 encoded files, returning an error otherwise.
pub fn read_file<T: AsRef<Path>>(path: T) -> Result<String, MawuError> {
    let contents = read_to_string(path.as_ref())
        .map_err(|e| MawuError::IoError(e))?;
    Ok(contents)
}
