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

    pub fn push_child(&mut self, n: Node) { 
        self.children.push(n);
    }
}

// validate the tree by checking each node: if it has left arrow, then it has a right arrow and nothing else in values
// same for parenthesis
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
}

pub fn print_tree(root: &Node, depth: u64) {
    let spacer = String::from("  ");
    let mut out = String::new();
    for _ in 0..depth {
        out.push_str(&spacer);
    }
    for v in root.values.iter() {
        out.push_str(&v.value);
        out.push_str(", ");
    }   
    println!("{}", out);
    for n in root.children.iter() {
        print_tree(n, depth+1);
    }
}

pub fn validate_tree(root: &Node) -> Result<String, String> {
    if root.values.len() > 0 {
        match root.values[0].kind {
            TokenType::LeftArrow => {
                if root.values.len() != 2 {
                    return Err(String::from("first"))
                } else if root.values[1].kind != TokenType::RightArrow {
                    return Err(String::from("second"))
                }
            },
            TokenType::LeftParen => {
                if root.values.len() != 2 {
                    return Err(String::from("third"));
                } else if root.values[1].kind != TokenType::RightParen {
                    return Err(String::from("fourth"))
                }
            },
            TokenType::RightArrow => return Err(String::from("fifth")),
            TokenType::RightParen => return Err(String::from("sixth")),
            _ => (),
        }
    }

    for n in root.children.iter() {
        match validate_tree(n) {
            Err(e) => return Err(e),
            _ => ()
        }
    }

    Ok(String::from("ok"))
}