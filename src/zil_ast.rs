use std::io;

use crate::tokenize::*;

pub struct Node {
    values: Vec<Token>,
    children: Vec<Node>,
}

impl Node {
    pub fn new() -> Node {
        return Node {values: Vec::new(), children: Vec::new()};
    }

    pub fn push_value(&mut self, t: Token) {
        self.values.push(t);
    }

    pub fn take_value(&mut self, i: usize) -> Option<Token> {
        if i >= self.values.len() {
            return None;
        } else {
            return Some(self.values.remove(i));
        }
    }

    pub fn push_child(&mut self, n: Node) { 
        self.children.push(n);
    }

    pub fn take_child(&mut self, i: usize) -> Option<Node> {
        if i >= self.children.len() {
            return None;
        } else {
            return Some(self.children.remove(i));
        }
    }
}

pub fn build_tree(tokens: &mut TokenGenerator, root: &mut Node) -> Option<io::Error> {
    while true {
        let t = match tokens.next() {
            Some(Ok(v)) => v,
            Some(Err(e)) => return Some(e),
            None => return None,
        };

        match t.kind {
            TokenType::LeftArrow | TokenType::LeftParen => {
                let mut child = Node::new();
                child.push_value(t);
                build_tree(tokens, &mut child);
                root.push_child(child);
            },
            TokenType::RightArrow | TokenType::RightParen => {
                root.push_value(t);
                return None;
            },
            TokenType::Text | TokenType::Word => {
                let mut child = Node::new();
                child.push_value(t);
                root.push_child(child);
            }
        }
    }

    return None;
}