use std::fmt;
use std::error::Error;

use crate::zil::contracts::*;
use crate::zil::tokenize::*;

#[derive(Copy, Clone, PartialEq)]
pub enum InterNodeType {
    Unknown,
    Routine,
    EmptyRoutine,
    Grouping,
    Text,
    Word,
    Int
}

impl fmt::Display for InterNodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl InterNodeType {
    pub fn to_str(&self) -> String {
        match *self {
            InterNodeType::Unknown => "InterNodeType::Unknown".to_string(),
            InterNodeType::Routine => "InterNodeType::Routine".to_string(),
            InterNodeType::EmptyRoutine => "InterNodeType::EmptyRoutine".to_string(),
            InterNodeType::Grouping => "InterNodeType::Grouping".to_string(),
            InterNodeType::Text => "InterNodeType::Text".to_string(),
            InterNodeType::Word => "InterNodeType::Word".to_string(),
            InterNodeType::Int => "InterNodeType::Int".to_string(),
        }
    }
}

pub struct InterNode {
    pub kind: InterNodeType,
    pub token: Option<Token>,
    pub children: Vec<InterNode>
}

impl fmt::Display for InterNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match &self.token {
        None => write!(f, "{}", &self.kind),
        Some(t) => write!(f, "{}: {}", &t.value, &self.kind)
      }
    }
}

impl InterNode {
    pub fn clone_zilnode(zn: &ZilNode) -> Result<InterNode, InterErr> {
        let mut start = 1;

        let kind: InterNodeType;
        let token: Option<Token>;
        match (zn.kind(), zn.children.len()) {
            (ZilNodeType::Routine, 0) => {
                kind = InterNodeType::EmptyRoutine;
                token = None;
            },
            (ZilNodeType::Routine, _) => {
                kind = InterNodeType::Routine;
                if !zn.children[0].is_word() {
                    return Err(InterErr::origin(format!("First child in routine is not a word. Near line {} in file {}", zn.tokens[0].file_key, zn.tokens[0].line_number)));
                }
                token = Some(zn.children[0].tokens[0].clone());
            },
            (ZilNodeType::Grouping, _) => {
                start = 0;
                kind = InterNodeType::Grouping;
                token = None;
            },
            (ZilNodeType::Text, 0) => {
                kind = InterNodeType::Text;
                token = Some(zn.tokens[0].clone());
            },
            (ZilNodeType::Word, 0) => {
                match zn.tokens[0].value.parse::<usize>() {
                  Ok(_) => { kind = InterNodeType::Int; },
                  Err(_) => { kind = InterNodeType::Word; }
                };
                token = Some(zn.tokens[0].clone());
            },
            (ZilNodeType::Unknown, _) => {
              kind = InterNodeType::Unknown;
              token = None;
            }
            _ => return Err(InterErr::origin("Unknown (ZilNodeType, children.len()) in InterNode::clone_zilnode"))
        };

        let mut children = Vec::new();
        for i in start..zn.children.len() {
            children.push(match Self::clone_zilnode(&zn.children[i]) {
                Ok(v) => v,
                Err(e) => return Err(InterErr::wrap(e, format!("{}", zn)))
            });
        }

        Ok(InterNode {
            kind: kind,
            token: token,
            children: children
        })
    }

    pub fn value(&self) -> &str {
      match &self.token {
        Some(t) => &t.value,
        None => ""
      }
    }
}

#[derive(Debug)]
pub struct InterErr {
  msg: String,
  from: Option<Box<InterErr>>
}

impl fmt::Display for InterErr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self.from {
      Some(b) => { write!(f, "{}", *b)?; },
      None => ()
    }
    write!(f, "{}", self.msg)
  }
}

impl Error for InterErr {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match &self.from {
      Some(b) => Some(b),
      None => None
    }
  }
}

impl InterErr {
  pub fn origin<S: Into<String>>(msg: S) -> InterErr {
    InterErr {
      msg: msg.into(),
      from: None
    }
  }

  pub fn wrap<S: Into<String>>(from: InterErr, msg: S) -> InterErr {
    InterErr {
      msg: msg.into(),
      from: Some(Box::new(from))
    }
  }
}