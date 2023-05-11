use std::fmt;

use super::token::Token;

#[derive(Copy, Clone, PartialEq)]
pub enum ZilNodeType {
    Unknown,
    Cluster,
    Group,
    Comment,
    TokenBunch,
    Token,
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
            ZilNodeType::Comment => "COMMENT".to_string(),
            ZilNodeType::TokenBunch => "TOKEN_BUNCH".to_string(),
            ZilNodeType::Token => "TOKEN".to_string(),
        }
    }
}

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
            out.push_str(&format!("token:\n  value: {}\n", self.token_val()));
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
    pub fn new(node_type: ZilNodeType) -> ZilNode {
        return ZilNode {
            node_type: node_type,
            token: None,
            children: Vec::new(),
        };
    }

    pub fn from_token(token: Token) -> ZilNode {
        return ZilNode {
            node_type: ZilNodeType::Token,
            token: Some(token),
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

    pub fn token_val(&self) -> String {
        self.token.as_ref().unwrap().value.clone()
    }
}
