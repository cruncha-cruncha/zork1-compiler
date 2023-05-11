use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ZilErr {
    msg: String,
    from: Option<Box<ZilErr>>,
}

impl fmt::Display for ZilErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.from {
            Some(b) => {
                write!(f, "{}", *b)?;
            }
            None => (),
        }
        write!(f, "{}", self.msg)
    }
}

impl Error for ZilErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.from {
            Some(b) => Some(b),
            None => None,
        }
    }
}

impl ZilErr {
    pub fn origin<S: Into<String>>(msg: S) -> ZilErr {
        ZilErr {
            msg: msg.into(),
            from: None,
        }
    }

    #[allow(dead_code)]
    pub fn wrap<S: Into<String>>(from: ZilErr, msg: S) -> ZilErr {
        ZilErr {
            msg: msg.into(),
            from: Some(Box::new(from)),
        }
    }
}
