pub mod errors;
mod lexers;
pub mod mawu_values;
mod utils;

pub mod read {
    use core::str;
    use std::{collections::VecDeque, path::Path};
    use unicode_segmentation::UnicodeSegmentation;

    use crate::{
        errors::MawuError,
        lexers::{csv_lexer, json_lexer},
        mawu_values::MawuValue,
        utils::file_handling,
    };

    /// Takes in a path to a CSV file with a header at the beginning of the file and returns a parsed MawuValue in the format of `Vec<Vec<HashMap<String, MawuValue>>>`.
    pub fn read_csv_headed<T: AsRef<Path>>(path: T) -> Result<MawuValue, MawuError> {
        csv_lexer::headed(
            file_handling::read_file(path)?
                .graphemes(true)
                .collect::<VecDeque<&str>>(),
        )
    }

    /// Takes in a path to a CSV file with no header at the beginning of the file and returns a parsed MawuValue in the format of `Vec<Vec<MawuValue>>`.
    pub fn read_csv_headless<T: AsRef<Path>>(path: T) -> Result<MawuValue, MawuError> {
        csv_lexer::headless(
            file_handling::read_file(path)?
                .graphemes(true)
                .collect::<VecDeque<&str>>(),
        )
    }

    pub fn read_json<T: AsRef<Path>>(path: T) -> Result<MawuValue, MawuError> {
        json_lexer::json_lexer(
            &mut file_handling::read_file(path)?
                .graphemes(true)
                .collect::<VecDeque<&str>>(),
        )
    }
}
