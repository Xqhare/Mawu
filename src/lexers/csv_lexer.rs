use std::collections::{HashMap, VecDeque};

use unicode_segmentation::UnicodeSegmentation;

use crate::{errors::{csv_error::{CsvError, CsvParseError}, MawuError}, mawu_values::MawuValue, utils::is_newline};


pub fn headed(input_string: String) -> Result<MawuValue, MawuError> {
    let mut file_contents = input_string.graphemes(true).collect::<VecDeque<&str>>();
    let head = {
        let mut head_done = false;
        let mut head_out: Vec<String> = Default::default();
        while !head_done {
            if let Some(h) = file_contents.pop_front() {
                if is_newline(h) {
                    head_done = true;
                } else if h == "," {
                    // do literally nothing
                } else {
                    if h == "\"" {
                        let mut value: String = Default::default();
                        let mut open_quote = true;
                        while open_quote {
                            if file_contents.front() == Some(&"\"") && file_contents.get(1) == Some(&"\"") {
                                value.push_str("\"");
                                let _ = file_contents.pop_front();
                                let _ = file_contents.pop_front();
                            } else if file_contents.front() == Some(&"\"") {
                                let _ = file_contents.pop_front();
                                open_quote = false;
                            } else {
                                if let Some(t) = file_contents.pop_front() {
                                    value.push_str(t);
                                }
                            }
                        }
                        head_out.push(value);
                    } else {
                        let mut value: String = h.to_string();
                        while file_contents.front() != Some(&",") && !is_newline(file_contents.front().ok_or_else(|| MawuError::CsvError(CsvError::ParseError(CsvParseError::UnexpectedNewline)))?) {
                            if let Some(t) = file_contents.pop_front() {
                                let mut entry = t.to_string();
                                while file_contents.front() != Some(&",") && !is_newline(file_contents.front().ok_or_else(|| MawuError::CsvError(CsvError::UnrecognizedHeader("".to_string())))?) {
                                    if let Some(g) = file_contents.pop_front() {
                                        entry.push_str(g);
                                    }
                                }
                                value.push_str(&entry);
                            }
                        }
                        head_out.push(value);
                    }
                }
            } else {
                let t = file_contents.iter().map(|s| format!("{}", s)).collect::<String>();
                return Err(MawuError::CsvError(CsvError::UnrecognizedHeader(t)));
            };
        };
        head_out
    };
    let body = parse_csv_body(file_contents)?;
    let mut out: Vec<HashMap<String, MawuValue>> = Default::default();
    for entry in body {
        let mut tmp_bind: HashMap<String, MawuValue> = Default::default();
        if entry.len() == head.len() {
            for (index, value) in entry.iter().enumerate() {
                tmp_bind.insert(head[index].clone(), value.clone());
            }
        } else if entry.len() > head.len() {
            return Err(MawuError::CsvError(CsvError::ParseError(CsvParseError::ExtraValue(format!("{:?}", entry)))));
        } else {
            return Err(MawuError::CsvError(CsvError::ParseError(CsvParseError::MissingValue(format!("{:?}", entry)))));
        };
        out.push(tmp_bind);
    }
    Ok(MawuValue::CSVObject(out))
}

pub fn headless(input_string: String) -> Result<MawuValue, MawuError> {
    todo!();
}

#[allow(unused_assignments)]
fn parse_csv_body(mut csv_body: VecDeque<&str>) -> Result<Vec<Vec<MawuValue>>, MawuError> {
    let mut out: Vec<Vec<MawuValue>> = Default::default();
    let mut row_data: Vec<MawuValue> = Default::default();
    while csv_body.front().is_some() {
        if let Some(h) = csv_body.pop_front() {
            if is_newline(h) {
                if is_newline(csv_body.front().ok_or_else(|| MawuError::CsvError(CsvError::ParseError(CsvParseError::UnexpectedNewline)))?) {
                    let _ = csv_body.pop_front();
                }          
                out.push(row_data);
                // assignment is only overwritten before being read if the very first character
                // is a newline and thus fine.
                row_data = Default::default();
            } else if h == "," {
                if is_newline(csv_body.front().ok_or_else(|| MawuError::CsvError(CsvError::ParseError(CsvParseError::UnexpectedNewline)))?) {
                    row_data.push(MawuValue::Null);
                }
            } else if h == "\"" {
                let mut value: String = Default::default();
                let mut open_quote = true;
                while open_quote {
                    if csv_body.front() == Some(&"\"") && csv_body.get(1) == Some(&"\"") {
                        value.push_str("\"");
                        let _ = csv_body.pop_front();
                        let _ = csv_body.pop_front();
                    } else if csv_body.front() == Some(&"\"") {
                        let _ = csv_body.pop_front();
                        open_quote = false;
                    } else {
                        if let Some(t) = csv_body.pop_front() {
                            value.push_str(t);
                        }
                    }
                }
                row_data.push(MawuValue::from(value));
                
            } else {
                let mut value: String = h.to_string();
                while csv_body.front() != Some(&",") && !is_newline(csv_body.front().ok_or_else(|| MawuError::CsvError(CsvError::ParseError(CsvParseError::UnexpectedNewline)))?) {
                    if let Some(t) = csv_body.pop_front() {
                        let mut entry = t.to_string();
                        while csv_body.front() != Some(&",") && !is_newline(csv_body.front().ok_or_else(|| MawuError::CsvError(CsvError::UnrecognizedHeader("".to_string())))?) {
                            if let Some(g) = csv_body.pop_front() {
                                entry.push_str(g);
                            }
                        }
                        value.push_str(&entry);
                    }
                }
                row_data.push(MawuValue::from(value.clone()));
            }
        }
    }
    Ok(out)
}
