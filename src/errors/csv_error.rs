use std::fmt;


pub enum CsvError {
    ParseError(CsvParseError),
}

pub type Result<T> = std::result::Result<T, CsvError>;

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CsvError::ParseError(ref e) => e.fmt(f),
        }
    }
    
}

enum CsvParseError {
    UnescapedDoubleQuote,
    UnterminatedQuote,
    UnescapedCharacter(char),
    UnrecognizedHeader(String),
    ExtraValue(String),
    MissingValue(String),
}

impl fmt::Display for CsvParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CsvParseError::UnescapedDoubleQuote => write!(f, "Unescaped double quote"),
            CsvParseError::UnterminatedQuote => write!(f, "Unterminated quote"),
            CsvParseError::UnescapedCharacter(c) => write!(f, "Unescaped character: {}", c),
            CsvParseError::UnrecognizedHeader(ref s) => write!(f, "Unrecognized header: {}", s),
            CsvParseError::ExtraValue(ref s) => write!(f, "Extra value: {}", s),
            CsvParseError::MissingValue(ref s) => write!(f, "Missing value: {}", s),
        }
    }
}
