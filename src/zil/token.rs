use std::fmt;
use zil::file_table::FileKey;

use super::file_table::FileTableLocation;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum TokenType {
    LeftArrow,
    RightArrow,
    LeftParen,
    RightParen,
    Text,  // " ... "
    Space, // any whitespace outside of text
    Word,  // not anything else
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl TokenType {
    pub fn to_str(&self) -> String {
        match *self {
            TokenType::LeftArrow => "L_ARROW".to_string(),
            TokenType::RightArrow => "R_ARROW".to_string(),
            TokenType::LeftParen => "L_PAREN".to_string(),
            TokenType::RightParen => "R_PAREN".to_string(),
            TokenType::Text => "TEXT".to_string(),
            TokenType::Space => "SPACE".to_string(),
            TokenType::Word => "WORD".to_string(),
        }
    }
}

pub struct Token {
    pub kind: TokenType,
    pub value: String,
    pub file_key: FileKey,
    pub line_number: u64,
    pub char_number: u64,
}

impl Token {
    #[allow(dead_code)]
    pub fn hard_copy(&self) -> Token {
        Token {
            kind: self.kind,
            value: self.value.clone(),
            file_key: self.file_key,
            line_number: self.line_number,
            char_number: self.char_number,
        }
    }
}

impl FileTableLocation for &Token {
    fn get_file_key(&self) -> FileKey {
        self.file_key
    }

    fn get_line_number(&self) -> u64 {
        self.line_number
    }

    fn get_char_number(&self) -> u64 {
        self.char_number
    }
}

pub fn word_is_integer(word: &str) -> bool {
    if word.len() < 1 {
        return false;
    }

    let first_char = word.chars().next().unwrap();
    if !first_char.is_digit(10) && first_char != '-' {
        return false;
    }

    for c in word.chars().skip(1) {
        if !c.is_digit(10) {
            return false;
        }
    }

    true
}
