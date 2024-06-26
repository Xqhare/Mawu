use std::{char, error::Error};

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

/// Takes in a `&str` and unescapes unicode characters
/// Expects `\` and `u` to be included
pub fn unescape_unicode(s: &str) -> Result<String, MawuError> {
    let out = my_unescape_unicode_handler(s.to_string());
    if out.is_err() {
        return Err(MawuError::InternalError(MawuInternalError::UnableToUnescapeUnicode(s.to_string())));
    } else {
        return Ok(out.unwrap());
    }
}

#[allow(unused_assignments)]
fn my_unescape_unicode_handler(s: String) -> Result<String, MawuError> {
    // To consider: Surrogate pairs - too much work for now; maybe in an actual unicode library in the future
    
    // This value is read, I don't know what the compiler is on about
    let mut unicode_value = 0u32;
    for char in s.chars() {
        let digit = char.to_digit(0x10);
        match digit {
            Some(d) => {
                // Just a casual bit-shift and a bitwise OR to build the unicode value
                unicode_value = (unicode_value << 4) + d;
                // Check if the unicode value is above 0x10FFFF (the maximum value of a unicode codepoint)
                if unicode_value > 0x10FFFF {
                    return Err(MawuError::InternalError(MawuInternalError::UnableToUnescapeUnicode(s.to_string())));
                }
            },
            // If the character is not a digit, it is an error!
            None => return Err(MawuError::InternalError(MawuInternalError::UnableToUnescapeUnicode(s.to_string()))),
        }
    }
    let possible_char = char::from_u32(unicode_value);
    // user supplied data always needs to be checked, invalid data can always be supplied
    if possible_char.is_none() {
        return Err(MawuError::InternalError(MawuInternalError::UnableToUnescapeUnicode(s.to_string())));
    } else {
        return Ok(possible_char.unwrap().to_string());
    }
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

/// Returns true if the given character is a json string terminator (':','}',']')
/// Do not forget to check for end of file!
/// Uses `\n` as end of file making it compatible with modern windows, linux and some OSX versions.
pub fn is_json_string_terminator_token(c: &str) -> bool {
    c == ":" || c == "," || c == "}" || c == "]" || c == "\n"
}
