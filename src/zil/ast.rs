use std::error::Error;
use std::io;
use std::fmt;

use crate::zil::tokenize::*;
use crate::zil::contracts::*;

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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        for t in &self.tokens {
            out.push_str(&format!("  Token: kind: {:>10}, value: {:>8}, file_key: {:>3}, line: {:>5}\n", &t.kind, &t.value, &t.line_number, &t.file_key));
        }
        write!(f, "Node\n{}", out)
    }
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

pub fn build_tree(mut tokens: &mut TokenGenerator, mut root: &mut Node) -> Result<(), Box<dyn Error>> {
    match build_tree_recursively(&mut tokens, &mut root) {
        Some(e) => return Err(Box::new(e)),
        None => ()
    };  

    match validate_tree(&root, 0) {
        Ok(()) => (),
        Err(e) => return Err(Box::new(e))
    }

    match remove_comments(&mut root) {
        Some(e) => return Err(Box::new(e)),
        None => ()
    };

    retain_child_routines(&mut root);

    remove_newlines(&mut root);

    Ok(())
}

fn build_tree_recursively(tokens: &mut TokenGenerator, root: &mut Node) -> Option<io::Error> {
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
                build_tree_recursively(tokens, &mut child);
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

fn validate_tree(root: &Node, depth: u64) -> Result<(), TVErr> {
    for n in root.children.iter() {
        match validate_tree(n, depth+1) {
            Ok(()) => (),
            Err(e) => return Err(TVErr::wrap(e, format!("from {}", n)))
        };
    }

    match root.tokens.len() {
        0 => {
            if depth != 0 {
                return Err(TVErr::origin("Root node has no tokens.\n"));
            }
        },
        1 => {
            match root.tokens[0].kind {
                TokenType::Text | TokenType::Word | TokenType::Comment => {
                    if root.children.len() > 0 {
                        return Err(TVErr::origin(format!("Text, Word, or Comment node has children.\nAt {}", root)));
                    }
                },
                _ => return Err(TVErr::origin(format!("Root node has only one token but it's not Text, Word, or Comment.\nAt {}", root))),
            }
        },
        2 => {
            match (root.tokens[0].kind, root.tokens[1].kind) {
                (TokenType::LeftArrow, TokenType::RightArrow) => {
                    if root.children.len() != 0 && !root.children[0].is_word() {
                        return Err(TVErr::origin(format!("Routine is not empty but does not start with a Word.\nAt {}", root)));
                    }
                },
                (TokenType::LeftParen, TokenType::RightParen) => (),
                _ => return Err(TVErr::origin(format!("Root node has two tokens but is not Routine or Grouping.\nAt {}", root))),
            }
        },
        x => return Err(TVErr::origin(format!("Root node has {} tokens; that's too many.\nAt {}", x, root))),
    }

    Ok(())
}

fn remove_comments(root: &mut Node) -> Option<TVErr> {
    let mut to_remove = Vec::new();
    for (i, n) in root.children.iter().enumerate() {
        if n.is_comment() {
            to_remove.push(i);
            to_remove.push(i+1);
        }
    }
    for i in to_remove.iter().rev() {
        if root.children.len() <= *i {
            return Some(TVErr::origin("Unable to remove comments: unexpected tree structure"));
        }
        root.children.remove(*i);
    }
    for i in 0..root.children.len() {
        match remove_comments(&mut root.children[i]) {
            Some(e) => return Some(e),
            None => ()
        }
    }
    None
}

// at the top level, we only care about things inside a <>
fn retain_child_routines(root: &mut Node) {
    root.children.retain(|n| n.is_routine());
}

fn remove_newlines(root: &mut Node) {
    let mut to_remove = Vec::new();
    for (i, n) in root.children.iter().enumerate() {
        if n.is_word() && n.tokens[0].value == String::from('\n') {
            to_remove.push(i);
        }
    }
    for i in to_remove.iter().rev() {
        root.children.remove(*i);
    }
    for i in 0..root.children.len() {
        remove_newlines(&mut root.children[i]);
    }
}