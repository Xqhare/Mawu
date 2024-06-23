use std::{collections::{HashMap, VecDeque}, rc::Rc, sync::{Mutex, MutexGuard}};

use crate::{
    errors::{
        json_error::{JsonError, JsonParseError}, MawuError, MawuInternalError
    },
    mawu_values::MawuValue,
    utils::{file_handling::is_digit, is_newline},
};

pub fn json_lexer(file_contents: &mut VecDeque<&str>) -> Result<MawuValue, MawuError> {

    let contents_store: Rc<Mutex<VecDeque<&str>>> = Rc::new(Mutex::new(file_contents.clone()));
    let mut contents = contents_store.lock().unwrap_or(Err(MawuError::InternalError(MawuInternalError::UnableToLockMasterMutex))?);
    if file_contents.len() > 0 {
        json_value_lexer(&mut contents)
    } else {
        Ok(MawuValue::default())
    }
}

fn json_value_lexer(file_contents: &mut MutexGuard<VecDeque<&str>>) -> Result<MawuValue, MawuError> {
    let this_char = file_contents.pop_front();
    if this_char.is_some() {
        let character = this_char.unwrap();
        if character == "{" {
            // object
            json_object_lexer(file_contents)
        } else if character == "[" {
            // array
            json_array_lexer(file_contents)
        } else if character == "t" && file_contents.front() == Some(&"r") && file_contents.get(1) == Some(&"u") && file_contents.get(2) == Some(&"e") {
            // true
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            Ok(MawuValue::Bool(true))
        } else if character == "f" && file_contents.front() == Some(&"a") && file_contents.get(1) == Some(&"l") && file_contents.get(2) == Some(&"s") && file_contents.get(3) == Some(&"e") {
            // false
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            Ok(MawuValue::Bool(false))
        } else if character == "n" && file_contents.front() == Some(&"u") && file_contents.get(1) == Some(&"l") && file_contents.get(2) == Some(&"l") {
            // null
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            let _ = file_contents.pop_front();
            Ok(MawuValue::None)
        } else if character == "}" || character == "]" || character == "," || character == ":" {
            // Invalid json grammar
            return Err(MawuError::JsonError(JsonError::ParseError(JsonParseError::InvalidStructuralToken(character.to_string()))));
        } else if character == "\"" {
            // string
            json_string_lexer(file_contents)
        } else if character == "-" || is_digit(character) {
            // number
            json_number_lexer(file_contents)
        } else {
            // Invalid json grammar
            return Err(MawuError::JsonError(JsonError::ParseError(JsonParseError::InvalidCharacter(character.to_string()))));
        }
    } else {
        Err(MawuError::JsonError(JsonError::ParseError(JsonParseError::UnexpectedEndOfFile)))
    }
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

fn json_number_lexer(file_contents: &mut MutexGuard<VecDeque<&str>>) -> Result<MawuValue, MawuError> {
    Ok(MawuValue::default())
}
