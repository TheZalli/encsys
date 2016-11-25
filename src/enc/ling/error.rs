use std::fmt;
use std::error::Error;

/// An error created by an operation of an `EncLing` `Encyclopedia`.
#[derive(Debug)]
pub enum EncLingError {
	FmtError(fmt::Error),
	InvalidTags, // TODO
}

impl fmt::Display for EncLingError {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		try!(f.write_str(self.description()));
		match self {
			&EncLingError::FmtError(ref e) => write!(f, ": {}", e),
			_ => Ok(())
		}
	}
}

impl Error for EncLingError {
	fn description(&self) -> &str {
		match self {
			&EncLingError::FmtError(_) => "format error",
			&EncLingError::InvalidTags => "invalid word tags",
		}
	}
}
