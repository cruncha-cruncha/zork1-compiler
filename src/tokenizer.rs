use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::LinkedList;

use crate::tokens_and_nodes::*;

pub fn tokenize (reader: BufReader<File>, token_names: &std::collections::HashMap<char, TokenType>) -> LinkedList<Token> {
    let mut out: LinkedList<Token> = LinkedList::new();
    let mut tmp_string = String::new();
    for line in reader.lines() {
        for c in line.unwrap().trim().chars() {
            match token_names.get(&c) {
                Some(n) => {
                    if tmp_string.len() > 0 {
                        out.push_back(Token {name: TokenType::Word, value: tmp_string.clone()});
                        tmp_string.clear();
                    }
                    out.push_back(Token {name: *n, value: c.to_string()});
                },
                None => {
                    tmp_string.push(c);
                }
            }
        }
        if tmp_string.len() > 0 {
            out.push_back(Token {name: TokenType::Word, value: tmp_string.clone()});
            tmp_string.clear();
        }
        out.push_back(Token {name: TokenType::Nl, value: "\n".to_string()});
    }

    out
}