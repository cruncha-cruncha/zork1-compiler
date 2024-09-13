use std::collections::VecDeque;
use std::io;
use zil::file_table::FileKey;

use crate::zil::file_table::format_file_location;

use super::char_gen::{CharGen, CharInfo};
use super::file_table::FileTableLocation;
use super::token::{Token, TokenType};

pub trait TokenGen: Iterator<Item = Result<Token, io::Error>> + FileTableLocation {}

pub struct TokenGenerator<'a> {
    char_gen: &'a mut dyn CharGen,
    out_buf: VecDeque<Token>,
    in_quotes: bool,
    text_buffer: TokenStrBuf,
    word_buffer: TokenStrBuf,
}

struct TokenStrBuf {
    val: String,
    has_info: bool,
    file_key: FileKey,
    line_number: u64,
    char_number: u64,
}

impl TokenStrBuf {
    pub fn new() -> TokenStrBuf {
        TokenStrBuf {
            val: String::new(),
            has_info: false,
            file_key: 0,
            line_number: 0,
            char_number: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.val.len()
    }

    pub fn has_info(&self) -> bool {
        self.has_info
    }

    pub fn get_val(&self) -> String {
        self.val.clone()
    }

    pub fn push(&mut self, c: char) {
        self.val.push(c);
    }

    pub fn set_info(&mut self, char_info: &CharInfo) {
        self.has_info = true;
        self.file_key = char_info.file_key;
        self.line_number = char_info.line_number;
        self.char_number = char_info.char_number;
    }

    pub fn clear(&mut self) {
        self.val.clear();
        self.has_info = false;
        self.file_key = 0;
        self.line_number = 0;
        self.char_number = 0;
    }
}

impl<'a> TokenGenerator<'a> {
    fn add_token_from_char(&mut self, kind: TokenType, char_info: &CharInfo) {
        self.out_buf.push_back(Token {
            kind,
            value: String::from(char_info.val),
            file_key: char_info.file_key,
            line_number: char_info.line_number,
            char_number: char_info.char_number,
        });
    }

    fn add_token_from_text_buffer(&mut self) {
        // empty string is valid

        self.out_buf.push_back(Token {
            kind: TokenType::Text,
            value: self.text_buffer.get_val(),
            file_key: self.text_buffer.file_key,
            line_number: self.text_buffer.line_number,
            char_number: self.text_buffer.char_number,
        });

        self.text_buffer.clear();
    }

    fn add_token_from_word_buffer(&mut self) {
        if self.word_buffer.len() <= 0 {
            return;
        }

        if self.word_buffer.val.chars().last().unwrap() == '-' {
            panic!("Unexpected dash\n{}", format_file_location(self));
        }

        self.out_buf.push_back(Token {
            kind: TokenType::Word,
            value: self.word_buffer.get_val(),
            file_key: self.word_buffer.file_key,
            line_number: self.word_buffer.line_number,
            char_number: self.word_buffer.char_number,
        });

        self.word_buffer.clear();
    }

    fn coerce_char_to_space(c: &CharInfo) -> CharInfo {
        CharInfo {
            val: ' ',
            file_key: c.file_key,
            line_number: c.line_number,
            char_number: c.char_number,
        }
    }
}

impl<'a> FileTableLocation for TokenGenerator<'a> {
    fn get_file_key(&self) -> FileKey {
        self.char_gen.get_file_key()
    }

    fn get_line_number(&self) -> u64 {
        self.char_gen.get_line_number()
    }

    fn get_char_number(&self) -> u64 {
        self.char_gen.get_char_number()
    }
}

impl<'a> Iterator for TokenGenerator<'a> {
    type Item = Result<Token, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.out_buf.len() > 0 {
            return Some(Ok(self.out_buf.pop_front().unwrap()));
        }

        let mut loop_count = 0;
        while self.out_buf.len() <= 0 {
            if self.in_quotes && loop_count > 5000 {
                panic!("String is too long\n{}", format_file_location(self));
            } else if !self.in_quotes && loop_count > 200 {
                panic!("Too many loops\n{}", format_file_location(self));
            }

            loop_count += 1;

            let char_info = match self.char_gen.next() {
                Some(Ok(v)) => v,
                Some(Err(e)) => return Some(Err(e)),
                None => {
                    if self.in_quotes {
                        panic!("Unexpected EOF\n{}", format_file_location(self));
                    } else {
                        // push the word buffer, call it a day
                        return None;
                    }
                }
            };

            let c = char_info.val;

            if self.in_quotes {
                match c {
                    '"' => {
                        self.add_token_from_text_buffer();
                        self.in_quotes = false;
                    }
                    '\n' => {
                        self.text_buffer.push(' ');
                    }
                    _ => self.text_buffer.push(c),
                }

                continue;
            }

            if c.is_whitespace() {
                self.add_token_from_word_buffer();
                self.add_token_from_char(TokenType::Space, &Self::coerce_char_to_space(&char_info));
                continue;
            }

            match c {
                '"' => {
                    self.add_token_from_word_buffer();
                    self.text_buffer.set_info(&char_info);
                    self.in_quotes = true;
                }
                '<' => {
                    self.add_token_from_word_buffer();
                    self.add_token_from_char(TokenType::LeftArrow, &char_info);
                }
                '>' => {
                    self.add_token_from_word_buffer();
                    self.add_token_from_char(TokenType::RightArrow, &char_info);
                }
                '(' => {
                    self.add_token_from_word_buffer();
                    self.add_token_from_char(TokenType::LeftParen, &char_info);
                }
                ')' => {
                    self.add_token_from_word_buffer();
                    self.add_token_from_char(TokenType::RightParen, &char_info);
                }
                _ => {
                    let mut good_char = false;
                    if c == '-' || c == ';' || c == '=' {
                        good_char = true;
                    } else if c.is_ascii_alphabetic() {
                        if c.to_ascii_uppercase() == c {
                            good_char = true;
                        }
                    } else if c.is_numeric() {
                        good_char = true;
                    }

                    if good_char {
                        self.word_buffer.push(c);
                        if !self.word_buffer.has_info() {
                            self.word_buffer.set_info(&char_info);
                        }
                    } else {
                        panic!("illegal character: {}\n{}", c, format_file_location(self));
                    }
                }
            }
        }

        Some(Ok(self.out_buf.pop_front().unwrap()))
    }
}

impl<'a> TokenGen for TokenGenerator<'a> {}

pub fn new<'a>(char_gen: &'a mut impl CharGen) -> TokenGenerator<'a> {
    TokenGenerator {
        char_gen,
        out_buf: VecDeque::new(),
        in_quotes: false,
        text_buffer: TokenStrBuf::new(),
        word_buffer: TokenStrBuf::new(),
    }
}
