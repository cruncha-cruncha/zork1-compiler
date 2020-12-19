use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fmt;
use std::collections::VecDeque;

#[derive(Copy, Clone, PartialEq)]
pub enum TokenType {
    LeftArrow,
    RightArrow,
    LeftParen,
    RightParen,
    Comment,
    Text,
    Word
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
            TokenType::Comment => "COMMENT".to_string(),
            TokenType::Word => "WORD".to_string(),
            TokenType::Text => "TEXT".to_string()
        }
    }
}

pub struct Token {
    pub kind: TokenType,
    pub value: String,
    pub file_key: u32,
    pub line_number: u64
}

struct CharGenerator {
    reader: BufReader<File>,
    char_buf: Vec<char>,
    buf_index: usize,
}

impl CharGenerator {
    pub fn new(reader: BufReader<File>) -> CharGenerator {
        CharGenerator {
            reader: reader,
            char_buf: Vec::new(),
            buf_index: 0,
        }
    }
}

impl Iterator for CharGenerator {
    type Item = Result<char, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf_index >= self.char_buf.len() {
            self.char_buf.clear();
        }
        while self.buf_index >= self.char_buf.len() {
            let mut line_buf = String::new();
            match self.reader.read_line(&mut line_buf) {
                Ok(0) => return None,
                Err(e) => return Some(Err(e)),
                Ok(_) => (),
            };
            self.buf_index = 0;
            self.char_buf.extend(line_buf.trim().chars());
            self.char_buf.push('\n');
        }

        self.buf_index += 1;
        Some(Ok(self.char_buf[self.buf_index-1]))
    }
}

pub struct TokenGenerator{
    char_gen: CharGenerator,
    str_buf: String,
    out_buf: VecDeque<Token>,
    file_key: u32,
    line_number: u64,
    in_string: bool,
    escape: u32
}

impl TokenGenerator {
    pub fn new(file_key: u32, reader: BufReader<File>) -> TokenGenerator {
        TokenGenerator {
            char_gen: CharGenerator::new(reader),
            str_buf: String::new(),
            out_buf: VecDeque::new(),
            file_key: file_key,
            line_number: 1,
            in_string: false,
            escape: 0,
        }
    }   
}

impl Iterator for TokenGenerator {
    type Item = Result<Token, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.out_buf.len() > 0 {
            return Some(Ok(self.out_buf.pop_front().unwrap()));
        }

        let loop_count = 0;
        while self.out_buf.len() <= 0 {
            let c = match self.char_gen.next() {
                Some(Ok(v)) => v,
                Some(Err(e)) => return Some(Err(e)),
                None => return None,
            };

            match c {
                '<' => {
                    if self.in_string {
                        self.str_buf.push('<');
                    } else {
                        if self.str_buf.trim() != "" {
                            self.out_buf.push_back(Token {
                                kind: TokenType::Word,
                                value: self.str_buf.to_string(),
                                file_key: self.file_key.clone(),
                                line_number: self.line_number});
                        }
                        self.out_buf.push_back(Token {
                            kind: TokenType::LeftArrow,
                            value: String::from("<"),
                            file_key: self.file_key.clone(),
                            line_number: self.line_number});
                        self.str_buf.clear();
                    }
                },
                '>' => {
                    if self.in_string {
                        self.str_buf.push('>');
                    } else {
                        if self.str_buf.trim() != "" {
                            self.out_buf.push_back(Token {
                                kind: TokenType::Word,
                                value: self.str_buf.to_string(),
                                file_key: self.file_key.clone(),
                                line_number: self.line_number});
                        }
                        self.out_buf.push_back(Token {
                            kind: TokenType::RightArrow,
                            value: String::from(">"),
                            file_key: self.file_key.clone(),
                            line_number: self.line_number});
                        self.str_buf.clear();
                    } 
                },
                '(' => {
                    if self.in_string {
                        self.str_buf.push('(');
                    } else {
                        if self.str_buf.trim() != "" {
                            self.out_buf.push_back(Token {
                                kind: TokenType::Word,
                                value: self.str_buf.to_string(),
                                file_key: self.file_key.clone(),
                                line_number: self.line_number});
                        }
                        self.out_buf.push_back(Token {
                            kind: TokenType::LeftParen,
                            value: String::from("("),
                            file_key: self.file_key.clone(),
                            line_number: self.line_number});
                        self.str_buf.clear();
                    } 
                },
                ')' => {
                    if self.in_string {
                        self.str_buf.push(')');
                    } else {
                        if self.str_buf.trim() != "" {
                            self.out_buf.push_back(Token {
                                kind: TokenType::Word,
                                value: self.str_buf.to_string(),
                                file_key: self.file_key.clone(),
                                line_number: self.line_number});
                        }
                        self.out_buf.push_back(Token {
                            kind: TokenType::RightParen,
                            value: String::from(")"),
                            file_key: self.file_key.clone(),
                            line_number: self.line_number});
                        self.str_buf.clear();
                    } 
                },
                '\\' => {
                    if self.in_string && self.escape <= 0 {
                        self.escape = 2;
                    } else {
                        self.str_buf.push('\\');
                    }
                },
                '"' => {
                    if self.in_string && self.escape > 0 {
                        self.str_buf.push('"');
                    } else if self.in_string {
                        self.out_buf.push_back(Token {
                            kind: TokenType::Text,
                            value: self.str_buf.to_string(),
                            file_key: self.file_key.clone(),
                            line_number: self.line_number});
                        self.str_buf.clear();
                        self.in_string = false;
                    } else {
                        if self.str_buf.trim() != "" {
                            self.out_buf.push_back(Token {
                                kind: TokenType::Word,
                                value: self.str_buf.to_string(),
                                file_key: self.file_key.clone(),
                                line_number: self.line_number});
                        }
                        self.str_buf.clear();
                        self.in_string = true;
                    }
                },
                ';' => {
                    if self.in_string {
                        self.str_buf.push(';');
                    } else {
                        if self.str_buf.trim() != "" {
                            self.out_buf.push_back(Token {
                                kind: TokenType::Word,
                                value: self.str_buf.to_string(),
                                file_key: self.file_key.clone(),
                                line_number: self.line_number}); 
                        }
                        self.out_buf.push_back(Token {
                            kind: TokenType::Comment,
                            value: String::from(";"),
                            file_key: self.file_key.clone(),
                            line_number: self.line_number});
                        self.str_buf.clear();
                    }
                }
                ' ' | '\t' => {
                    if self.in_string {
                        self.str_buf.push(' ');
                    } else {
                        if self.str_buf.trim() != "" {
                            self.out_buf.push_back(Token {
                                kind: TokenType::Word,
                                value: self.str_buf.to_string(),
                                file_key: self.file_key.clone(),
                                line_number: self.line_number});
                        }
                        self.str_buf.clear();
                    }
                },
                '\n' => {
                    if self.in_string {
                        self.str_buf.push('\n');
                    } else {
                        if self.str_buf.trim() != "" {
                            self.out_buf.push_back(Token {
                                kind: TokenType::Word,
                                value: self.str_buf.to_string(),
                                file_key: self.file_key.clone(),
                                line_number: self.line_number});
                        }
                        self.str_buf.clear();
                        self.out_buf.push_back(Token {
                            kind: TokenType::Word,
                            value: String::from("\n"),
                            file_key: self.file_key.clone(),
                            line_number: self.line_number});
                    }
                    self.line_number += 1;
                },
                x => self.str_buf.push(x)
            }

            if self.escape > 0 {
                self.escape -= 1;
            }

            if self.in_string && loop_count > 5000 {
                println!("{}", self.str_buf);
                println!("near line: {}", self.line_number);
                panic!("String is too long");
            } else if loop_count > 200 {
                println!("{}", self.str_buf);
                println!("near line: {}", self.line_number);
                panic!("Too many loops");
            }
        }

        Some(Ok(self.out_buf.pop_front().unwrap()))
    }
}