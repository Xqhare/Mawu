use std::error::Error;

use crate::errors::{MawuError, MawuInternalError};

pub mod file_handling;

/// Takes in a `&str` and checks if it is a newline character
/// (any of `\n`, `\r\n`, `\r`)
///
/// ## Returns
/// `true` if the string is a newline, `false` otherwise
pub fn is_newline(s: &str) -> bool {
    s == "\n" || s == "\r\n" || s == "\r"
}

pub fn unescape_unicode(s: &[u8]) -> Result<String, Box<dyn Error>> {
    let mut output = Vec::new();
    let mut i = 0;
    
    while i < s.len() {
        match s[i] {
            b'\\' => {
                i += 1;
                match s[i] {
                    b'u' => {
                        let num = u8::from_str_radix(std::str::from_utf8(&s[i+1..][..4])?, 16)?;
                        output.push(num);
                        i += 4;
                    }
                    byte => output.push(byte),
                }
            },
            byte => output.push(byte),
        }
        i += 1;
    }

    Ok(String::from_utf8(output)?)
}

/// Takes in a `&str` and checks the very first character to see if it is a digit
///
/// ## Returns
/// `true` if the first character is a digit, `false` otherwise
///
/// ## Errors
/// `MawuError::InternalError` if the string has no characters
pub fn is_digit(c: &str) -> Result<bool, MawuError> {
    let charr = c.chars().next();
    if charr.is_some() {
        // This if loop has proven to be faster than the char method `is_digit` by a very slight
        // margin. But it is faster! (Using `match` is slower than both `if` and `char` methods)
        if charr.unwrap() >= '0' && charr.unwrap() <= '9' {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Err(MawuError::InternalError(MawuInternalError::StringWithNoChars(c.to_string())))
    }
}

pub fn is_end_of_primitive_value(c: &str) -> bool {
    c == "," || c == ":" || c == "}" || c == "]"
}

pub fn is_whitespace(c: &str) -> bool {
    is_newline(c) || c == " " || c == "\t"
}
