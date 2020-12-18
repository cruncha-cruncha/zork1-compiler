use std::error::Error;
use std::fmt;

use crate::trace_error::TraceError;

#[derive(Debug)]
pub struct TVErr { // Tree Validation Error
  msg: String,
  from: Option<Box<TVErr>>
}

impl Error for TVErr {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match &self.from {
      Some(b) => Some(b),
      None => None
    }
  }
}

impl fmt::Display for TVErr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl TraceError for TVErr {
  fn sheath(sword: TVErr, mut scabard: TVErr) -> TVErr {
    scabard.from = Some(Box::from(sword));
    scabard
  }
}

impl TVErr {
  pub fn new(msg: String) -> TVErr {
    TVErr {
      msg: msg,
      from: None
    }
  }
}

