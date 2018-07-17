use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughChars,
    UnrecognizedQuote,
    UnexpectedEOF,
    IllegalChar,
    UnrecognizedEscape,
    InvalidUnicode,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::NotEnoughChars => "not enough chars",
            Error::UnrecognizedQuote => "unrecognized quote character",
            Error::UnexpectedEOF => "unexpected eof",
            Error::IllegalChar => "illegal character",
            Error::UnrecognizedEscape => "unrecognized escape sequence",
            Error::InvalidUnicode => "invalid unicode code point",
        }
    }
}
