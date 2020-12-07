use std::io;

use crate::zil::tokenize::*;

#[derive(Copy, Clone, PartialEq)]
pub enum NodeType {
    Routine,
    Grouping,
    Comment,
    Text,
    Word
}

pub struct Node {
    pub tokens: Vec<Token>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new() -> Node {
        return Node {tokens: Vec::new(), children: Vec::new()};
    }

    pub fn push_token(&mut self, t: Token) {
        self.tokens.push(t);
    }

    pub fn kind(&self) -> NodeType {
        if self.is_routine() {
            NodeType::Routine
        } else if self.is_grouping() {
            NodeType::Grouping
        } else if self.is_comment() {
            NodeType::Comment
        } else if self.is_text() {
            NodeType::Text
        } else if self.is_word() {
            NodeType::Word
        } else {
            panic!("Don't know what NodeType");
        }
    }

    pub fn push_child(&mut self, n: Node) { 
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

pub fn build_tree(tokens: &mut TokenGenerator, root: &mut Node) -> Option<io::Error> {
    loop {
        let t = match tokens.next() {
            Some(Ok(v)) => v,
            Some(Err(e)) => return Some(e),
            None => return None,
        };

        match t.kind {
            TokenType::LeftArrow | TokenType::LeftParen => {
                let mut child = Node::new();
                child.push_token(t);
                build_tree(tokens, &mut child);
                root.push_child(child);
            },
            TokenType::RightArrow | TokenType::RightParen => {
                root.push_token(t);
                return None;
            },
            TokenType::Comment => {
                let mut child = Node::new();
                child.push_token(t);
                root.push_child(child);
            },
            TokenType::Text | TokenType::Word => {
                let mut child = Node::new();
                child.push_token(t);
                root.push_child(child);
            }
        }
    }
}

// at the top level, we only care about things inside a <>
pub fn retain_child_routines(root: &mut Node) {
    root.children.retain(|n| n.is_routine());
}

pub fn remove_comments(root: &mut Node) {
    let mut to_remove = Vec::new();
    for (i, n) in root.children.iter().enumerate() {
        if n.is_comment() {
            to_remove.push(i);
            to_remove.push(i+1);
        }
    }
    for i in to_remove.iter().rev() {
        root.children.remove(*i);
    }
    for i in 0..root.children.len() {
        remove_comments(&mut root.children[i]);
    }
}

pub fn print_tree(root: &Node, depth: u64) {
    let spacer = String::from("  ");
    let mut out = String::new();
    for _ in 0..depth {
        out.push_str(&spacer);
    }
    for t in root.tokens.iter() {
        out.push_str(&t.value);
        out.push_str(", ");
    }   
    println!("{}", out);
    for n in root.children.iter() {
        print_tree(n, depth+1);
    }
}

pub fn validate_tree(root: &Node, depth: u64) -> Result<(), ()> {
    match root.tokens.len() {
        0 => {
            if depth != 0 {
                return Err(());
            }
        },
        1 => {
            match root.tokens[0].kind {
                TokenType::Text | TokenType::Word => {
                    if root.children.len() > 0 {
                        return Err(());
                    }
                },
                _ => return Err(()),
            }
        },
        2 => {
            match (root.tokens[0].kind, root.tokens[1].kind) {
                (TokenType::LeftArrow, TokenType::RightArrow) => {
                    if root.children.len() != 0 && !root.children[0].is_word() {
                        return Err(());
                    }
                },
                (TokenType::LeftParen, TokenType::RightParen) => {
                    if root.children.len() == 1 {
                        return Err(());
                    }
                },
                _ => return Err(()),
            }
        },
        _ => return Err(()),
    }

    for n in root.children.iter() {
        match validate_tree(n, depth+1) {
            Err(_) => return Err(()),
            _ => ()
        }
    }

    Ok(())
}