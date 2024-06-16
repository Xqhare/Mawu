use std::fmt;

#[derive(Debug)]
pub enum CsvError {
    ParseError(CsvParseError),
    UnrecognizedHeader(String),
}

pub type Result<T> = std::result::Result<T, CsvError>;

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CsvError::ParseError(ref e) => e.fmt(f),
            CsvError::UnrecognizedHeader(ref s) => write!(f, "Unrecognized header: {}", s),
        }
    }
}

#[derive(Debug)]
pub enum CsvParseError {
    UnescapedDoubleQuote,
    UnterminatedQuote,
    UnescapedCharacter(char),
    ExtraValue(String),
    MissingValue(String),
    UnexpectedNewline,
}

impl fmt::Display for CsvParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CsvParseError::UnescapedDoubleQuote => write!(f, "Unescaped double quote"),
            CsvParseError::UnterminatedQuote => write!(f, "Unterminated quote"),
            CsvParseError::UnescapedCharacter(c) => write!(f, "Unescaped character: {}", c),
            CsvParseError::ExtraValue(ref s) => write!(f, "Extra value: {}", s),
            CsvParseError::MissingValue(ref s) => write!(f, "Missing value: {}", s),
            CsvParseError::UnexpectedNewline => write!(f, "Unexpected newline"),
        }
    }
}
