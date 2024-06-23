use std::collections::{HashMap, VecDeque};

use crate::{
    errors::{
        csv_error::{CsvError, CsvParseError},
        MawuError,
    },
    mawu_values::MawuValue,
    utils::is_newline,
};

pub fn json_lexer(file_contents: &mut VecDeque<&str>) -> Result<MawuValue, MawuError> {

    if file_contents.len() > 0 {
        json_value_lexer(file_contents)
    } else {
        Ok(MawuValue::default())
    }
}

fn json_value_lexer(file_contents: &mut VecDeque<&str>) -> Result<MawuValue, MawuError> {
    while file_contents.front().is_some() {
        let character = file_contents.pop_front().unwrap();
        match character {
            "{" => ,
            "[" => ,
            "t" => ,
            "f" => ,
            "n" => ,
            "}" || "]" || "," || ":" => MawuError::
}
