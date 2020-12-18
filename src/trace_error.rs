use std::error::Error;

pub trait TraceError<E: Error> {
  fn sheath(line: u32, e: E) -> E; // format!("line: {}, file: {}", line!, file!);
  fn catch(line: u32, r: Result<(), E>) -> Result<(), E> {
    match r {
      Ok(v) => Ok(v),
      Err(e) => Err(Self::sheath(line, e)),
    }
  }
}