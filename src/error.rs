use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct Error {
	pub message: String,
}

impl Error {
	pub fn new(msg: String) -> Error {
		Error { message: msg }
	}
}

impl StdError for Error {
	fn description(&self) -> &str {
		&self.message
	}
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}