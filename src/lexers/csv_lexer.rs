use std::collections::{HashMap, VecDeque};


use crate::{errors::{csv_error::{CsvError, CsvParseError}, MawuError}, mawu_values::MawuValue, utils::is_newline};

pub fn headed(file_contents: VecDeque<&str>) -> Result<MawuValue, MawuError> {
    let (head, left_content) = make_head(file_contents)?;
    let body = parse_csv_body(left_content, head.len())?;
    let mut out: Vec<HashMap<String, MawuValue>> = Default::default();
    for entry in body {
        let mut tmp_bind: HashMap<String, MawuValue> = Default::default();
        if entry.len() == head.len() {
            for (index, value) in entry.iter().enumerate() {
                tmp_bind.insert(head[index].clone(), value.clone());
            }
        } else {
            return Err(MawuError::CsvError(CsvError::ParseError(CsvParseError::ExtraValue(format!("{:?}", entry)))));
        };
        out.push(tmp_bind);
    }
    Ok(MawuValue::CSVObject(out))
}

pub fn headless(file_contents: VecDeque<&str>) -> Result<MawuValue, MawuError> {
    let (head, left_content) = make_head(file_contents)?;
    let mut body = parse_csv_body(left_content, head.len())?;
    body.insert(0, head.into_iter().map(|s| MawuValue::from(s)).collect::<Vec<MawuValue>>());
    Ok(MawuValue::CSVArray(body))
}

#[allow(unused_assignments)]
fn parse_csv_body(mut csv_body: VecDeque<&str>, head_length: usize) -> Result<Vec<Vec<MawuValue>>, MawuError> {
    let mut out: Vec<Vec<MawuValue>> = Default::default();
    let mut row_data: Vec<MawuValue> = Default::default();
    while csv_body.front().is_some() {
        if let Some(h) = csv_body.pop_front() {
            if is_newline(h) {
                if csv_body.is_empty() || is_newline(csv_body.front().ok_or_else(|| MawuError::CsvError(CsvError::ParseError(CsvParseError::UnexpectedNewline)))?) {
                    let _ = csv_body.pop_front();
                }          
                out.push(row_data);
                // assignment is only overwritten before being read if the very first character IS a newline and thus, probably, maybe, fine.
                row_data = Default::default();
            } else if h == "," {
                if is_newline(csv_body.front().ok_or_else(|| MawuError::CsvError(CsvError::ParseError(CsvParseError::UnexpectedNewline)))?) {
                    // push as many nulls as needed to fill in the missing data
                    if head_length > row_data.len() {
                        for _ in 0..(head_length - row_data.len()) {
                            row_data.push(MawuValue::Null);
                        }
                    }
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

fn make_head(mut file_contents: VecDeque<&str>) -> Result<(Vec<String>, VecDeque<&str>), MawuError> {
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
    Ok((head_out, file_contents))
}

