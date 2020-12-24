use std::fmt;
use std::error::Error;

use crate::zil::contracts::{ZilNode, ZilNodeType};

#[derive(Copy, Clone, PartialEq)]
pub enum InterNodeType {
    Routine,
    EmptyRoutine,
    Grouping,
    EmptyGrouping,
    Text,
    Word
}

impl fmt::Display for InterNodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl InterNodeType {
    pub fn to_str(&self) -> String {
        match *self {
            InterNodeType::Routine => "InterNodeType::Routine".to_string(),
            InterNodeType::EmptyRoutine => "InterNodeType::EmptyRoutine".to_string(),
            InterNodeType::Grouping => "InterNodeType::Grouping".to_string(),
            InterNodeType::EmptyGrouping => "InterNodeType::EmptyGrouping".to_string(),
            InterNodeType::Text => "InterNodeType::Text".to_string(),
            InterNodeType::Word => "InterNodeType::Word".to_string(),
        }
    }
}

pub struct InterNode {
    pub kind: InterNodeType,
    pub value: String,
    pub children: Vec<InterNode>
}

impl fmt::Display for InterNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", &self.value, &self.kind)
    }
}

impl InterNode {
    fn clone_zilnode(zn: &ZilNode) -> Result<Self, InterErr> {
        let kind: InterNodeType;
        let value: String;
        match (zn.kind(), zn.children.len()) {
            (ZilNodeType::Routine, 0) => {
                kind = InterNodeType::EmptyRoutine;
                value = String::from("");
            }
            (ZilNodeType::Routine, _) => {
                kind = InterNodeType::Routine;
                if !zn.children[0].is_word() {
                    return Err(InterErr::origin(format!("First child in routine is not a word. Near line {} in file {}", zn.tokens[0].file_key, zn.tokens[0].line_number)));
                }
                value = String::from(&zn.children[0].tokens[0].value);
            }
            (ZilNodeType::Grouping, 0) => {
                kind = InterNodeType::EmptyGrouping;
                value = String::from("");
            }
            (ZilNodeType::Grouping, _) => {
                kind = InterNodeType::Grouping;
                if !zn.children[0].is_word() {
                    return Err(InterErr::origin(format!("First child in grouping is not a word. Near line {} in file {}", zn.tokens[0].file_key, zn.tokens[0].line_number)));
                }
                value = String::from(&zn.children[0].tokens[0].value);
            }
            (ZilNodeType::Text, 0) => {
                kind = InterNodeType::Text;
                value = String::from(&zn.tokens[0].value);
            }
            (ZilNodeType::Word, 0) => {
                kind = InterNodeType::Word;
                value = String::from(&zn.tokens[0].value);
            }
            _ => return Err(InterErr::origin("Unkown (ZilNodeType, children.len()) in InterNode::clone_zilnode"))
        };

        let mut children = Vec::new();
        for c in zn.children.iter() {
            children.push(match Self::clone_zilnode(&c) {
                Ok(v) => v,
                Err(e) => return Err(InterErr::wrap(e, format!("{}", zn)))
            });
        }

        Ok(InterNode {
            kind: kind,
            value: value,
            children: children
        })
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