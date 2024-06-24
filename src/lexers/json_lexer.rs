use std::{collections::{HashMap, VecDeque}, rc::Rc, sync::{Mutex, MutexGuard}};

use crate::{
    errors::{
        json_error::{JsonError, JsonParseError}, MawuError, MawuInternalError
    },
    mawu_values::MawuValue,
    utils::{is_digit, is_end_of_primitive_value, is_whitespace},
};

pub fn json_lexer(file_contents: &mut VecDeque<&str>) -> Result<MawuValue, MawuError> {

    let contents_store: Rc<Mutex<VecDeque<&str>>> = Rc::new(Mutex::new(file_contents.clone()));
    if file_contents.len() > 0 {
        let contents = contents_store.try_lock();
        if contents.is_err() {
            return Err(MawuError::InternalError(MawuInternalError::UnableToLockMasterMutex));
        } else {
            json_value_lexer(&mut contents.unwrap())
        }
    } else {
        Ok(MawuValue::default())
    }
}
fn json_value_lexer(file_contents: &mut MutexGuard<VecDeque<&str>>) -> Result<MawuValue, MawuError> {
    while file_contents.len() > 0 {
        let this_char = file_contents.pop_front().unwrap();
        // Ignore whitespace
        if is_whitespace(this_char) {
            continue;
        }
        // Actual parsing
        if this_char == "{" {
            // object
            return json_object_lexer(file_contents);
        } else if this_char == "[" {
            // array
            return json_array_lexer(file_contents);
        } else if this_char == "t" && file_contents.front() == Some(&"r") && file_contents.get(1) == Some(&"u") && file_contents.get(2) == Some(&"e") {
            // true
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            return Ok(MawuValue::Bool(true));
        } else if this_char == "f" && file_contents.front() == Some(&"a") && file_contents.get(1) == Some(&"l") && file_contents.get(2) == Some(&"s") && file_contents.get(3) == Some(&"e") {
            // false
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            return Ok(MawuValue::Bool(false));
        } else if this_char == "n" && file_contents.front() == Some(&"u") && file_contents.get(1) == Some(&"l") && file_contents.get(2) == Some(&"l") {
            // null
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            return Ok(MawuValue::None);
        } else if this_char == "}" || this_char == "]" || this_char == "," || this_char == ":" {
            // Invalid json grammar
            return Err(MawuError::JsonError(JsonError::ParseError(JsonParseError::InvalidStructuralToken(this_char.to_string()))));
        } else if this_char == "\"" {
            // string
            return json_string_lexer(file_contents);
        } else if this_char == "-" || is_digit(this_char)? {
            // number
            if this_char == "-" {
                return json_number_lexer(file_contents, true, None);
            } else {
                return json_number_lexer(file_contents, false, Some(this_char));
            }
        } else {
            // Invalid json grammar
            return Err(MawuError::JsonError(JsonError::ParseError(JsonParseError::InvalidCharacter(this_char.to_string()))));
        }
    }
    Err(MawuError::JsonError(JsonError::ParseError(JsonParseError::UnexpectedEndOfFile)))
}

fn json_object_lexer(file_contents: &mut MutexGuard<VecDeque<&str>>) -> Result<MawuValue, MawuError> {
    Ok(MawuValue::default())
}

fn json_array_lexer(file_contents: &mut MutexGuard<VecDeque<&str>>) -> Result<MawuValue, MawuError> {
    Ok(MawuValue::default())
}

fn json_string_lexer(file_contents: &mut MutexGuard<VecDeque<&str>>) -> Result<MawuValue, MawuError> {
    Ok(MawuValue::default())
}

fn json_number_lexer(file_contents: &mut MutexGuard<VecDeque<&str>>, is_negative: bool, first_digit: Option<&str>) -> Result<MawuValue, MawuError> {
    let mut out: String = Default::default();
    if first_digit.is_some() {
        out.push_str(first_digit.unwrap());
    } else if is_negative {
        out.push_str("-");
    }
    while file_contents.len() > 0 {
        let this_char = file_contents.pop_front().unwrap();
        if this_char == "." || is_digit(this_char)? {
            out.push_str(this_char);
        } else if this_char == "e" || this_char == "E" {
            out.push_str(this_char);
            if file_contents.front() == Some(&"+") || file_contents.front() == Some(&"-") {
                out.push_str(file_contents.pop_front().unwrap());
            } else if is_digit(file_contents.front().unwrap())? {
                out.push_str("+");
            } else {
                return Err(MawuError::JsonError(JsonError::ParseError(JsonParseError::InvalidCharacter(this_char.to_string()))));
            }
        } else if is_end_of_primitive_value(this_char) {
            break;
        } else {
            return Err(MawuError::JsonError(JsonError::ParseError(JsonParseError::InvalidCharacter(this_char.to_string()))));
        }
    }
    Ok(MawuValue::from(out))
}

#[test]
fn number_lexer() {
    let mut easy_neg = VecDeque::from(vec!["-","1","2","3"]);
    let easy_neg_res = json_lexer(&mut easy_neg).unwrap();
    assert_eq!(easy_neg_res, MawuValue::from("-123"));
    let mut easy_pos = VecDeque::from(vec!["1","2","3"]);
    let easy_pos_res = json_lexer(&mut easy_pos).unwrap();
    assert_eq!(easy_pos_res, MawuValue::from("123"));

    let mut large_neg = VecDeque::from(vec!["-","9","8","7","6","5","4","3","2","1"]);
    let large_neg_res = json_lexer(&mut large_neg).unwrap();
    assert_eq!(large_neg_res, MawuValue::from("-987654321"));
    let mut large_pos = VecDeque::from(vec!["9","8","7","6","5","4","3","2","1"]);
    let large_pos_res = json_lexer(&mut large_pos).unwrap();
    assert_eq!(large_pos_res, MawuValue::from("987654321"));

    let mut easy_float = VecDeque::from(vec!["1",".","2","3"]);
    let easy_float_res = json_lexer(&mut easy_float).unwrap();
    assert_eq!(easy_float_res, MawuValue::from("1.23"));
    let mut easy_neg_float = VecDeque::from(vec!["-","1",".","2","3"]);
    let easy_neg_float_res = json_lexer(&mut easy_neg_float).unwrap();
    assert_eq!(easy_neg_float_res, MawuValue::from("-1.23"));

    let mut large_float = VecDeque::from(vec!["9",".","8","7","6","5","4","3","2","1"]);
    let large_float_res = json_lexer(&mut large_float).unwrap();
    assert_eq!(large_float_res, MawuValue::from("9.87654321"));
    let mut large_neg_float = VecDeque::from(vec!["-","9",".","8","7","6","5","4","3","2","1"]);
    let large_neg_float_res = json_lexer(&mut large_neg_float).unwrap();
    assert_eq!(large_neg_float_res, MawuValue::from("-9.87654321"));

    let mut easy_exp = VecDeque::from(vec!["1",".","2","3","e","+","1","2"]);
    let easy_exp_res = json_lexer(&mut easy_exp).unwrap();
    assert_eq!(easy_exp_res, MawuValue::from("1230000000000.0"));
    let mut easy_neg_exp = VecDeque::from(vec!["-","1",".","2","3","e","+","1","2"]);
    let easy_neg_exp_res = json_lexer(&mut easy_neg_exp).unwrap();
    assert_eq!(easy_neg_exp_res, MawuValue::from("-1230000000000.0"));

    let mut large_exp = VecDeque::from(vec!["9",".","8","7","6","5","4","3","2","1","e","+","1","2"]);
    let large_exp_res = json_lexer(&mut large_exp).unwrap();
    assert_eq!(large_exp_res, MawuValue::from("9876543210000.0"));
    let mut large_neg_exp = VecDeque::from(vec!["-","9",".","8","7","6","5","4","3","2","1","e","1","2"]);
    let large_neg_exp_res = json_lexer(&mut large_neg_exp).unwrap();
    assert_eq!(large_neg_exp_res, MawuValue::from("-9876543210000.0"));

    let mut neg_exp = VecDeque::from(vec!["1",".","2","3","e","-","9"]);
    let neg_exp_res = json_lexer(&mut neg_exp).unwrap();
    println!("{:?}", neg_exp_res);
    assert_eq!(neg_exp_res, MawuValue::from("0.00000000123"));
    let mut neg_exp2 = VecDeque::from(vec!["-","1",".","2","3","e","-","1","2"]);
    let neg_exp2_res = json_lexer(&mut neg_exp2).unwrap();
    assert_eq!(neg_exp2_res, MawuValue::from("-0.00000000000123"));
}
