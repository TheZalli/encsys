use std::fmt;
use std::error::Error;

/// An error created by an operation of an `EncLing` `Encyclopedia`.
#[derive(Debug)]
pub enum LingError {
    FmtError(fmt::Error),
    InvalidTags, // TODO
    NoSuchGrammCat(String),
    NoSuchGrammeme(String),
    NoDefaultOrValue(String),
}

impl fmt::Display for LingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // let's reuse the description function
        try!(f.write_str(self.description()));
        // add possible additional information
        match self {
            &LingError::FmtError(ref e) => write!(f, ": {}", e),
            &LingError::NoSuchGrammCat(ref s) => write!(f, ": {}", s),
            &LingError::NoSuchGrammeme(ref s) => write!(f, ": {}", s),
            &LingError::NoDefaultOrValue(ref s) => write!(f, ": {}", s),
            _ => Ok(()),
        }
    }
}

impl Error for LingError {
    fn description(&self) -> &str {
        match self {
            &LingError::FmtError(_) => "format error",
            &LingError::InvalidTags => "invalid word tags",
            &LingError::NoSuchGrammCat(_) => "grammatical category not found",
            &LingError::NoSuchGrammeme(_) => "grammeme value not found",
            &LingError::NoDefaultOrValue(_) => {
                "no grammeme value was given and there is no default value for the category"
            }
        }
    }
}
