use std::fmt;

use super::{
    file_table::{FileKey, FileTableLocation},
    token::Token,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ZilNodeType {
    Unknown,
    Cluster,
    Group,
    Token(TokenType),
    Space,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum TokenType {
    Text,
    Word,
    Number,
}

impl fmt::Display for ZilNodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl ZilNodeType {
    pub fn to_str(&self) -> String {
        match *self {
            ZilNodeType::Unknown => "UNKNOWN".to_string(),
            ZilNodeType::Cluster => "CLUSTER".to_string(),
            ZilNodeType::Group => "GROUP".to_string(),
            ZilNodeType::Token(t) => match t {
                TokenType::Text => "TOKEN(TEXT)".to_string(),
                TokenType::Word => "TOKEN(WORD)".to_string(),
                TokenType::Number => "TOKEN(NUMBER)".to_string(),
            },
            ZilNodeType::Space => "SPACE".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct ZilNode {
    pub node_type: ZilNodeType,
    pub token: Option<Token>,
    pub children: Vec<ZilNode>,
}

impl fmt::Display for ZilNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        out.push_str(&format!("kind: {}\n", self.node_type));
        if self.token.is_none() {
            out.push_str("no token\n");
        } else {
            out.push_str(&format!(
                "token:\n  value: {}\n",
                self.token_val().unwrap_or_default()
            ));
        }
        if self.children.len() > 0 {
            out.push_str("has children\n");
        } else {
            out.push_str("no children\n");
        }
        write!(f, "ZilNode\n{}", out)
    }
}

impl ZilNode {
    pub fn new(node_type: ZilNodeType, token: Token) -> ZilNode {
        return ZilNode {
            node_type,
            token: Some(token),
            children: Vec::new(),
        };
    }

    pub fn new_no_token(node_type: ZilNodeType) -> ZilNode {
        return ZilNode {
            node_type,
            token: None,
            children: Vec::new(),
        };
    }

    pub fn push_child(&mut self, n: ZilNode) {
        self.children.push(n);
    }

    pub fn has_token(&self) -> bool {
        !self.token.is_none()
    }

    pub fn has_children(&self) -> bool {
        self.children.len() > 0
    }

    pub fn token_val(&self) -> Option<String> {
        match self.token {
            Some(ref t) => Some(t.value.clone()),
            None => None,
        }
    }

    pub fn get_first_token(&self) -> Option<&Token> {
        if !self.token.is_none() {
            return self.token.as_ref();
        }

        for c in self.children.iter() {
            match c.get_first_token() {
                Some(t) => {
                    return Some(t);
                }
                None => (),
            }
        }

        None
    }
}

impl FileTableLocation for &ZilNode {
    fn get_file_key(&self) -> FileKey {
        match self.get_first_token() {
            Some(token) => token.get_file_key(),
            None => 0,
        }
    }

    fn get_line_number(&self) -> u64 {
        match self.get_first_token() {
            Some(token) => token.get_line_number(),
            None => 0,
        }
    }

    fn get_char_number(&self) -> u64 {
        match self.get_first_token() {
            Some(token) => token.get_char_number(),
            None => 0,
        }
    }
}

impl FileTableLocation for &mut ZilNode {
    fn get_file_key(&self) -> FileKey {
        match self.get_first_token() {
            Some(token) => token.get_file_key(),
            None => 0,
        }
    }

    fn get_line_number(&self) -> u64 {
        match self.get_first_token() {
            Some(token) => token.get_line_number(),
            None => 0,
        }
    }

    fn get_char_number(&self) -> u64 {
        match self.get_first_token() {
            Some(token) => token.get_char_number(),
            None => 0,
        }
    }
}

impl FileTableLocation for ZilNode {
    fn get_file_key(&self) -> FileKey {
        match self.get_first_token() {
            Some(token) => token.get_file_key(),
            None => 0,
        }
    }

    fn get_line_number(&self) -> u64 {
        match self.get_first_token() {
            Some(token) => token.get_line_number(),
            None => 0,
        }
    }

    fn get_char_number(&self) -> u64 {
        match self.get_first_token() {
            Some(token) => token.get_char_number(),
            None => 0,
        }
    }
}
