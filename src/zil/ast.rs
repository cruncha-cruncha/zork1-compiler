use std::error::Error;
use std::io;
use std::fmt;

use crate::zil::tokenize::*;
use crate::zil::contracts::*;

pub fn print_tree(root: &ZilNode, depth: u64) {
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

pub fn build_tree(mut tokens: &mut TokenGenerator, mut root: &mut ZilNode) -> Result<(), Box<dyn Error>> {
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

fn build_tree_recursively(tokens: &mut TokenGenerator, root: &mut ZilNode) -> Option<io::Error> {
    loop {
        let t = match tokens.next() {
            Some(Ok(v)) => v,
            Some(Err(e)) => return Some(e),
            None => return None,
        };

        match t.kind {
            TokenType::LeftArrow | TokenType::LeftParen => {
                let mut child = ZilNode::new();
                child.push_token(t);
                build_tree_recursively(tokens, &mut child);
                root.push_child(child);
            },
            TokenType::RightArrow | TokenType::RightParen => {
                root.push_token(t);
                return None;
            },
            TokenType::Comment => {
                let mut child = ZilNode::new();
                child.push_token(t);
                root.push_child(child);
            },
            TokenType::Text | TokenType::Word => {
                let mut child = ZilNode::new();
                child.push_token(t);
                root.push_child(child);
            }
        }
    }
}

fn validate_tree(root: &ZilNode, depth: u64) -> Result<(), ZilErr> {
    for n in root.children.iter() {
        match validate_tree(n, depth+1) {
            Ok(()) => (),
            Err(e) => return Err(ZilErr::wrap(e, format!("from {}", n)))
        };
    }

    match root.tokens.len() {
        0 => {
            if depth != 0 {
                return Err(ZilErr::origin("Root ZilNode has no tokens.\n"));
            }
        },
        1 => {
            match root.tokens[0].kind {
                TokenType::Text | TokenType::Word | TokenType::Comment => {
                    if root.children.len() > 0 {
                        return Err(ZilErr::origin(format!("Text, Word, or Comment ZilNode has children.\nAt {}", root)));
                    }
                },
                _ => return Err(ZilErr::origin(format!("Root ZilNode has only one token but it's not Text, Word, or Comment.\nAt {}", root))),
            }
        },
        2 => {
            match (root.tokens[0].kind, root.tokens[1].kind) {
                (TokenType::LeftArrow, TokenType::RightArrow) => {
                    if root.children.len() != 0 && !root.children[0].is_word() {
                        return Err(ZilErr::origin(format!("Routine is not empty but does not start with a Word.\nAt {}", root)));
                    }
                },
                (TokenType::LeftParen, TokenType::RightParen) => (),
                _ => return Err(ZilErr::origin(format!("Root ZilNode has two tokens but is not Routine or Grouping.\nAt {}", root))),
            }
        },
        x => return Err(ZilErr::origin(format!("Root ZilNode has {} tokens; that's too many.\nAt {}", x, root))),
    }

    Ok(())
}

fn remove_comments(root: &mut ZilNode) -> Option<ZilErr> {
    let mut to_remove = Vec::new();
    for (i, n) in root.children.iter().enumerate() {
        if n.is_comment() {
            to_remove.push(i);
            to_remove.push(i+1);
        }
    }
    for i in to_remove.iter().rev() {
        if root.children.len() <= *i {
            return Some(ZilErr::origin("Unable to remove comments: unexpected tree structure"));
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
fn retain_child_routines(root: &mut ZilNode) {
    root.children.retain(|n| n.is_routine());
}

fn remove_newlines(root: &mut ZilNode) {
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