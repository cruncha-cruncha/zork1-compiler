use std::error::Error;
use std::fmt;

use crate::zil::tokenize::*;

#[derive(Copy, Clone, PartialEq)]
pub enum ZilNodeType {
    Routine,
    Grouping,
    Comment,
    Text,
    Word
}

pub struct ZilNode {
    pub tokens: Vec<Token>,
    pub children: Vec<ZilNode>,
}

impl fmt::Display for ZilNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        for t in &self.tokens {
            out.push_str(&format!("  Token: kind: {:>10}, value: {:>8}, file_key: {:>3}, line: {:>5}\n", &t.kind, &t.value, &t.line_number, &t.file_key));
        }
        write!(f, "ZilNode\n{}", out)
    }
}

impl ZilNode {
    pub fn new() -> ZilNode {
        return ZilNode {tokens: Vec::new(), children: Vec::new()};
    }

    pub fn push_token(&mut self, t: Token) {
        self.tokens.push(t);
    }

    pub fn kind(&self) -> ZilNodeType {
        if self.is_routine() {
            ZilNodeType::Routine
        } else if self.is_grouping() {
            ZilNodeType::Grouping
        } else if self.is_comment() {
            ZilNodeType::Comment
        } else if self.is_text() {
            ZilNodeType::Text
        } else if self.is_word() {
            ZilNodeType::Word
        } else {
            panic!("Don't know what ZilNodeType");
        }
    }

    pub fn push_child(&mut self, n: ZilNode) { 
        self.children.push(n);
    }

    pub fn is_routine(&self) -> bool {
        self.tokens.len() == 2 &&
        self.tokens[0].kind == TokenType::LeftArrow &&
        self.tokens[1].kind == TokenType::RightArrow
    }

    pub fn is_grouping(&self) -> bool {
        self.tokens.len() == 2 &&
        self.tokens[0].kind == TokenType::LeftParen &&
        self.tokens[1].kind == TokenType::RightParen
    }

    pub fn is_comment(&self) -> bool {
        self.tokens.len() == 1 &&
        self.tokens[0].kind == TokenType::Comment
    }

    pub fn is_text(&self) -> bool {
        self.tokens.len() == 1 &&
        self.tokens[0].kind == TokenType::Text
    }

    pub fn is_word(&self) -> bool {
        self.tokens.len() == 1 &&
        self.tokens[0].kind == TokenType::Word
    }

    pub fn has_children(&self) -> bool {
        self.children.len() > 0
    }
}

#[derive(Debug)]
pub struct ZilErr {
  msg: String,
  from: Option<Box<ZilErr>>
}

impl fmt::Display for ZilErr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self.from {
      Some(b) => { write!(f, "{}", *b)?; },
      None => ()
    }
    write!(f, "{}", self.msg)
  }
}

impl Error for ZilErr {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match &self.from {
      Some(b) => Some(b),
      None => None
    }
  }
}

impl ZilErr {
  pub fn origin<S: Into<String>>(msg: S) -> ZilErr {
    ZilErr {
      msg: msg.into(),
      from: None
    }
  }

  pub fn wrap<S: Into<String>>(from: ZilErr, msg: S) -> ZilErr {
    ZilErr {
      msg: msg.into(),
      from: Some(Box::new(from))
    }
  }
}