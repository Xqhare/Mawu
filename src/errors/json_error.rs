use std::fmt;

#[derive(Debug)]
pub enum JsonError {
    ParseError(JsonParseError),
    UnrecognizedHeader(String),
}

pub type Result<T> = std::result::Result<T, JsonError>;

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JsonError::ParseError(ref e) => e.fmt(f),
            JsonError::UnrecognizedHeader(ref s) => write!(f, "Unrecognized header: {}", s),
        }
    }
}

#[derive(Debug)]
pub enum JsonParseError {
    UnescapedDoubleQuote,
    UnterminatedQuote,
    UnescapedCharacter(char),
    ExtraValue(String),
    MissingValue(String),
    UnexpectedNewline,
    UnexpectedEndOfFile,
    UnexpectedCharacter(String),
    InvalidStructuralToken(String),
    InvalidCharacter(String),
    InvalidEscapeSequence(String),
    ExpectedColon,
    ExpectedKey,
    ExpectedValue,
    ExpectedEndOfObject,
}

impl fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JsonParseError::UnescapedDoubleQuote => write!(f, "Unescaped double quote"),
            JsonParseError::UnterminatedQuote => write!(f, "Unterminated quote"),
            JsonParseError::UnescapedCharacter(c) => write!(f, "Unescaped character: {}", c),
            JsonParseError::ExtraValue(ref s) => write!(f, "Extra value: {}", s),
            JsonParseError::MissingValue(ref s) => write!(f, "Missing value: {}", s),
            JsonParseError::UnexpectedNewline => write!(f, "Unexpected newline"),
            JsonParseError::InvalidStructuralToken(ref s) => {
                write!(f, "Invalid structural token: {}", s)
            }
            JsonParseError::UnexpectedEndOfFile => write!(f, "Unexpected end of file"),
            JsonParseError::InvalidCharacter(ref s) => write!(f, "Invalid character: {}", s),
            JsonParseError::InvalidEscapeSequence(ref s) => {
                write!(f, "Invalid escape sequence: {}", s)
            }
            JsonParseError::ExpectedColon => write!(f, "Expected colon"),
            JsonParseError::ExpectedKey => write!(f, "Expected key"),
            JsonParseError::ExpectedValue => write!(f, "Expected value"),
            JsonParseError::UnexpectedCharacter(ref s) => write!(f, "Unexpected character: {}", s),
            JsonParseError::ExpectedEndOfObject => write!(f, "Expected end of object"),
        }
    }
}
