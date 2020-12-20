use std::fs::File;
use std::io;
use std::fmt;
use std::error::Error;

use crate::zil::ast::Node;
use crate::js::custom_buf_writer::*;

pub trait HandleJS {
  fn validate (root: &Node) -> Result<(), HandlerErr>;
  fn print (root: &Node, indent: u64, writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr>;
}

#[derive(Debug)]
pub struct HandlerErr {
    msg: String,
    from: Option<Box<OutputErr>>
}

impl fmt::Display for HandlerErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.from {
            Some(b) => { 
                // because we're borrowing self.from, b is a reference to a Box (b: &Box<OutputErr>)
                // we have to dereference to get the Box (*b), dereference to get it's value (**b), and then borrow the value (&**b)
                match &**b {
                    OutputErr::System(e) => { write!(f, "{}", e)?; },
                    OutputErr::Semantic(e) => { write!(f, "{}", e)?; },
                }
            },
            None => (),
        }
        write!(f, "{}", self.msg)
    }
}
  
impl Error for HandlerErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.from {
            Some(b) => {
                match &**b {
                    OutputErr::System(e) => Some(e),
                    OutputErr::Semantic(e) => Some(e)
                }
            }
            None => None
        }
    }
}

impl HandlerErr {
  pub fn origin<S: Into<String>>(msg: S) -> HandlerErr {
    HandlerErr {
      msg: msg.into(),
      from: None
    }
  }

  pub fn wrap<S: Into<String>>(from: OutputErr, msg: S) -> HandlerErr {
    HandlerErr {
      msg: msg.into(),
      from: Some(Box::new(from))
    }
  }
}

#[derive(Debug)]
pub enum OutputErr {
    System(io::Error),
    Semantic(HandlerErr),
}

impl From<io::Error> for OutputErr {
    fn from(e: io::Error) -> Self {
        OutputErr::System(e)
    }
}

impl From<HandlerErr> for OutputErr {
    fn from(e: HandlerErr) -> Self {
        OutputErr::Semantic(e)
    }
}

impl fmt::Display for OutputErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            OutputErr::System(e) => write!(f, "{}", e),
            OutputErr::Semantic(e) => write!(f, "{}", e),
        }
    }
}