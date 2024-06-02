mod utils;
mod lexers;
pub mod errors;
pub mod mawu_values;

pub mod csv {
    use std::path::Path;

    use crate::{errors::MawuError, lexers::csv_lexer, mawu_values::MawuValue, utils::file_handling};

    pub fn from_csv_headed<T: AsRef<Path>>(path: T) -> Result<MawuValue, MawuError> {
        let file = file_handling::read_file(path)?;
        csv_lexer::headed(file)
    }

    pub fn from_csv_headless<T: AsRef<Path>>(path: T) -> Result<MawuValue, MawuError> {
        let file = file_handling::read_file(path)?;
        csv_lexer::headless(file)
    }

    pub fn from_csv_malformed<T: AsRef<Path>>(path: T) -> Result<MawuValue, MawuError> {
        from_csv_headless(path)
    }
}

