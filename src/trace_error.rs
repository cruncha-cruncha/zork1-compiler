use std::error::Error;

pub trait TraceError<E=Self> {
  fn sheath(sword: E, scabard: E) -> E;
  fn trace(r: Result<(), E>, wrapper: E) -> Result<(), E> {
    match r {
      Ok(v) => Ok(v),
      Err(e) => Err(Self::sheath(e, wrapper)),
    }
  }
}