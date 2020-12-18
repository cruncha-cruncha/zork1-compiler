use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ValidationError {}

impl Error for ValidationError {}

impl fmt::Display for ValidationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Validation error")
  }
}

impl ValidationError {
  pub fn new() -> ValidationError {
    ValidationError {}
  }
}

