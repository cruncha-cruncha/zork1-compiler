use std::collections::VecDeque;
use std::io;
use zil::file_table::FileKey;

use super::char_gen::{CharGen, CharInfo};
use super::file_table::{FileTableLocation, FileTableLocationString};
use super::token::{Token, TokenType};

pub trait TokenGen:
    Iterator<Item = Result<Token, io::Error>> + FileTableLocation + FileTableLocationString
{
}

struct TokenGenerator<'a> {
    char_gen: Box<dyn CharGen + 'a>,
    out_buf: VecDeque<Token>,
    in_word: bool,
    in_quotes: bool,
    escape: bool,
    str_buf: TokenStrBuf,
}

struct TokenStrBuf {
    val: String,
    file_key: FileKey,
    line_number: u64,
    char_number: u64,
}

impl TokenStrBuf {
    pub fn len(&self) -> usize {
        self.val.len()
    }

    pub fn clear(&mut self) {
        self.val.clear();
        self.file_key = 0;
        self.line_number = 0;
        self.char_number = 0;
    }
}

impl<'a> TokenGenerator<'a> {
    fn push_buf(&mut self, kind: TokenType) {
        if self.str_buf.len() > 0 {
            self.out_buf.push_back(Token {
                kind,
                value: self.str_buf.val.clone(),
                file_key: self.str_buf.file_key,
                line_number: self.str_buf.line_number,
                char_number: self.str_buf.char_number,
            });
            self.str_buf.clear();
        }
    }

    fn push_char(&mut self, token_type: TokenType, char_info: &CharInfo) {
        self.out_buf.push_back(Token {
            kind: token_type,
            value: String::from(char_info.val),
            file_key: char_info.file_key,
            line_number: char_info.line_number,
            char_number: char_info.char_number,
        });
    }

    fn buffer(&mut self, char_info: &CharInfo) {
        if self.str_buf.len() <= 0 {
            self.str_buf.file_key = char_info.file_key;
            self.str_buf.line_number = char_info.line_number;
        }

        self.str_buf.val.push(char_info.val);
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

impl<'a> FileTableLocationString for TokenGenerator<'a> {
    fn get_location_string(&self) -> String {
        self.char_gen.get_location_string()
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
                panic!(
                    "String is too long\n{}",
                    self.char_gen.get_location_string()
                );
            } else if !self.in_quotes && loop_count > 200 {
                panic!("Too many loops\n{}", self.char_gen.get_location_string());
            }

            loop_count += 1;

            let char_info = match self.char_gen.next() {
                Some(Ok(v)) => v,
                Some(Err(e)) => return Some(Err(e)),
                None => {
                    if self.in_quotes {
                        panic!("Unexpected EOF\n{}", self.char_gen.get_location_string());
                    } else if self.in_word {
                        self.push_buf(TokenType::Word);
                        self.in_word = false;
                        continue;
                    } else {
                        return None;
                    }
                }
            };

            let c = char_info.val;

            if c == '\\' {
                self.escape = true;
                continue;
            }

            if self.in_quotes {
                match c {
                    '"' => {
                        if self.escape {
                            self.buffer(&char_info);
                            self.escape = false;
                        } else {
                            self.push_buf(TokenType::Text);
                            self.in_quotes = false;
                        }
                    }
                    _ => self.buffer(&char_info),
                }

                continue;
            }

            if self.escape || c.is_alphanumeric() {
                self.buffer(&char_info);
                self.in_word = true;
                self.escape = false;
                continue;
            }

            if self.in_word {
                self.push_buf(TokenType::Word);
                self.in_word = false;
            }

            if c.is_whitespace() {
                // all whitespace outside of double quotes is coerced to a single space
                self.push_char(
                    TokenType::Space,
                    &CharInfo {
                        val: ' ',
                        file_key: char_info.file_key,
                        line_number: char_info.line_number,
                        char_number: char_info.char_number,
                    },
                );
                continue;
            }

            match c {
                '"' => {
                    self.in_quotes = true;
                }
                '<' => {
                    self.push_char(TokenType::LeftArrow, &char_info);
                }
                '>' => {
                    self.push_char(TokenType::RightArrow, &char_info);
                }
                '(' => {
                    self.push_char(TokenType::LeftParen, &char_info);
                }
                ')' => {
                    self.push_char(TokenType::RightParen, &char_info);
                }
                _ => {
                    self.push_char(TokenType::Symbol, &char_info);
                }
            }
        }

        Some(Ok(self.out_buf.pop_front().unwrap()))
    }
}

impl<'a> TokenGen for TokenGenerator<'a> {}

pub fn new<'a>(char_gen: Box<dyn CharGen + 'a>) -> Box<dyn TokenGen + 'a> {
    Box::new(TokenGenerator {
        char_gen,
        out_buf: VecDeque::new(),
        in_word: false,
        in_quotes: false,
        escape: false,
        str_buf: TokenStrBuf {
            val: String::new(),
            file_key: 0,
            line_number: 0,
            char_number: 0,
        },
    })
}
