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
    in_escape: bool,
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

        self.out_buf.push_back(Token {
            kind: TokenType::Word,
            value: self.word_buffer.get_val(),
            file_key: self.word_buffer.file_key,
            line_number: self.word_buffer.line_number,
            char_number: self.word_buffer.char_number,
        });

        self.word_buffer.clear();
    }

    // fn push_buf(&mut self, kind: TokenType) {
    //     if self.str_buf.len() > 0 {
    //         self.out_buf.push_back(Token {
    //             kind,
    //             value: self.str_buf.val.clone(),
    //             file_key: self.str_buf.file_key,
    //             line_number: self.str_buf.line_number,
    //             char_number: self.str_buf.char_number,
    //         });
    //         self.str_buf.clear();
    //     }
    // }

    // fn push_char(&mut self, token_type: TokenType, char_info: &CharInfo) {
    //     self.out_buf.push_back(Token {
    //         kind: token_type,
    //         value: String::from(char_info.val),
    //         file_key: char_info.file_key,
    //         line_number: char_info.line_number,
    //         char_number: char_info.char_number,
    //     });
    // }

    // fn buffer(&mut self, char_info: &CharInfo) {
    //     if self.str_buf.len() <= 0 {
    //         self.str_buf.file_key = char_info.file_key;
    //         self.str_buf.line_number = char_info.line_number;
    //     }

    //     self.str_buf.val.push(char_info.val);
    // }
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

// impl<'a> Iterator for TokenGenerator<'a> {
//     type Item = Result<Token, io::Error>;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.out_buf.len() > 0 {
//             return Some(Ok(self.out_buf.pop_front().unwrap()));
//         }

//         let mut loop_count = 0;
//         while self.out_buf.len() <= 0 {
//             if self.in_quotes && loop_count > 5000 {
//                 panic!("String is too long\n{}", format_file_location(self));
//             } else if !self.in_quotes && loop_count > 200 {
//                 panic!("Too many loops\n{}", format_file_location(self));
//             }

//             loop_count += 1;

//             let char_info = match self.char_gen.next() {
//                 Some(Ok(v)) => v,
//                 Some(Err(e)) => return Some(Err(e)),
//                 None => {
//                     if self.in_quotes {
//                         panic!("Unexpected EOF\n{}", format_file_location(self));
//                     } else if self.in_word {
//                         self.push_buf(TokenType::Char);
//                         self.in_word = false;
//                         continue;
//                     } else {
//                         return None;
//                     }
//                 }
//             };

//             let c = char_info.val;

//             if c == '\\' {
//                 self.escape = true;
//                 continue;
//             }

//             if self.in_quotes {
//                 match c {
//                     '"' => {
//                         if self.escape {
//                             let mut fake_char_info = char_info.clone();
//                             fake_char_info.val = '\\';
//                             self.buffer(&fake_char_info);
//                             fake_char_info.val = '"';
//                             self.buffer(&fake_char_info);
//                             self.escape = false;
//                         } else {
//                             self.push_buf(TokenType::Text);
//                             self.in_quotes = false;
//                         }
//                     }
//                     _ => {
//                         if c == '|' || c == '\n' {
//                             // coerce to a space
//                             let mut fake_char_info = char_info.clone();
//                             fake_char_info.val = ' ';
//                             self.buffer(&fake_char_info);
//                         } else {
//                             self.buffer(&char_info);
//                         }
//                     }
//                 }

//                 continue;
//             }

//             if self.escape || c.is_alphanumeric() {
//                 self.buffer(&char_info);
//                 self.in_word = true;
//                 self.escape = false;

//                 if c.is_alphanumeric() && c.is_lowercase() {
//                     panic!("Can't have lowercase here\n{}", format_file_location(self));
//                 }

//                 continue;
//             }

//             if self.in_word {
//                 self.push_buf(TokenType::Word);
//                 self.in_word = false;
//             }

//             if c.is_whitespace() {
//                 // all whitespace outside of double quotes is coerced to a single space
//                 self.push_char(
//                     TokenType::Space,
//                     &CharInfo {
//                         val: ' ',
//                         file_key: char_info.file_key,
//                         line_number: char_info.line_number,
//                         char_number: char_info.char_number,
//                     },
//                 );
//                 continue;
//             }

//             match c {
//                 '"' => {
//                     self.in_quotes = true;
//                 }
//                 '<' => {
//                     self.push_char(TokenType::LeftArrow, &char_info);
//                 }
//                 '>' => {
//                     self.push_char(TokenType::RightArrow, &char_info);
//                 }
//                 '(' => {
//                     self.push_char(TokenType::LeftParen, &char_info);
//                 }
//                 ')' => {
//                     self.push_char(TokenType::RightParen, &char_info);
//                 }
//                 _ => {
//                     self.push_char(TokenType::Symbol, &char_info);
//                 }
//             }
//         }

//         Some(Ok(self.out_buf.pop_front().unwrap()))
//     }
// }

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
                        if self.in_escape {
                            self.text_buffer.push(c);
                            self.in_escape = false;
                        } else {
                            self.add_token_from_text_buffer();
                            self.in_quotes = false;
                        }
                    }
                    '\\' => {
                        if self.in_escape {
                            self.text_buffer.push(c);
                            self.in_escape = false;
                        } else {
                            self.in_escape = true;
                        }
                    }
                    _ => {
                        if c == '|' {
                            self.text_buffer.push('\n');
                        } else {
                            self.text_buffer.push(c);
                        }
                    }
                }

                continue;
            }

            // if self.in_escape || c.is_alphanumeric() {
            //     self.buffer(&char_info);
            //     self.in_word = true;
            //     self.escape = false;

            //     if c.is_alphanumeric() && c.is_lowercase() {
            //         panic!("Can't have lowercase here\n{}", format_file_location(self));
            //     }

            //     continue;
            // }

            // if self.in_word {
            //     self.push_buf(TokenType::Word);
            //     self.in_word = false;
            // }

            if c.is_whitespace() {
                self.add_token_from_word_buffer();
                self.add_token_from_char(TokenType::Space, &char_info);
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
                    self.word_buffer.push(c);
                    if !self.word_buffer.has_info() {
                        self.word_buffer.set_info(&char_info);
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
        in_escape: false,
        text_buffer: TokenStrBuf::new(),
        word_buffer: TokenStrBuf::new(),
    }
}
