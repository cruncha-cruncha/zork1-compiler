use std::io::BufWriter;
use std::io::Write;

use crate::js::contracts::OutputErr;

pub struct CustomBufWriter<T: Write> {
  writer: BufWriter<T>
}

impl<T: Write> CustomBufWriter<T> {
  pub fn new (input: T) -> CustomBufWriter<T> {
    CustomBufWriter {
      writer: BufWriter::new(input)
    }
  }

  pub fn w<S: Into<String>>(&mut self, s: S) -> Result<(), OutputErr> {
    match self.writer.write(s.into().as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
  }
}