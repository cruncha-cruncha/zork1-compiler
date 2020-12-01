use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io;

// tokenizer should get rid of comments

#[derive(Copy, Clone)]
pub enum TokenType {
    LeftArrow,
    RightArrow,
    LeftParen,
    RightParen,
    Word,
    Text
}

impl TokenType {
    pub fn to_str(&self) -> String {
        match *self {
            TokenType::LeftArrow => "L_ARROW".to_string(),
            TokenType::RightArrow => "R_ARROW".to_string(),
            TokenType::LeftParen => "L_PAREN".to_string(),
            TokenType::RightParen => "R_PAREN".to_string(),
            TokenType::Word => "WORD".to_string(),
            TokenType::Text => "TEXT".to_string()
        }
    }
}

pub struct Token {
    pub kind: TokenType,
    pub value: String,
    pub line_number: u64
}

pub fn tokenize(reader: BufReader<File>) -> Result<Vec<Token>, io::Error>  {
    let mut out = Vec::new();
    let mut line_number = 1;
    let mut in_comment = false;
    let mut in_string = false;
    let mut escape = 0;
    for maybe_line in reader.lines() {
        let spacey_line = match maybe_line {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let line = spacey_line.trim();
        if line.starts_with(";") {
            in_comment = true;
        }

        //line.push(" ");
        let mut buf = String::new();
        for c in line.chars() {
            match c {
                '<' => {
                    if in_string || in_comment {
                        buf.push('<');
                    } else {
                        out.push(Token {kind: TokenType::LeftArrow, value: String::from("<"), line_number: line_number});
                    }
                },
                '>' => {
                    if in_string || in_comment {
                        buf.push('<');
                    } else {
                        if buf.trim() != "" {
                            out.push(Token {kind: TokenType::Word, value: buf.to_string(), line_number: line_number});
                        }
                        out.push(Token {kind: TokenType::RightArrow, value: String::from(">"), line_number: line_number});
                        buf.clear();
                    } 
                },
                '(' => {
                    if in_string || in_comment {
                        buf.push('<');
                    } else {
                        out.push(Token {kind: TokenType::LeftParen, value: String::from("("), line_number: line_number});
                    } 
                },
                ')' => {
                    if in_string || in_comment {
                        buf.push('<');
                    } else {
                        if buf.trim() != "" {
                            out.push(Token {kind: TokenType::Word, value: buf.to_string(), line_number: line_number});
                        }
                        out.push(Token {kind: TokenType::RightParen, value: String::from(")"), line_number: line_number});
                        buf.clear();
                    } 
                },
                '\\' => {
                    if in_string && escape <= 0 {
                        escape = 2;
                    } else {
                        buf.push('\\');
                    }
                },
                '"' => {
                    if in_string && escape > 0 {
                        buf.push('"');
                    } else if in_string && in_comment {
                        buf.clear();
                        in_comment = false;
                        in_string = false;
                    } else if in_string {
                        out.push(Token {kind: TokenType::Text, value: buf.to_string(), line_number: line_number});
                        buf.clear();
                        in_string = false;
                    } else {
                        buf.clear();
                        in_string = true;
                    }
                },
                ';' => {
                    if in_string {
                        buf.push(';');
                    } else {
                        in_comment = true;
                    }
                }
                ' ' => {
                    if in_string {
                        buf.push(' ');
                    } else if !in_comment {
                        if buf.trim() != "" {
                            out.push(Token {kind: TokenType::Word, value: buf.to_string(), line_number: line_number});
                        }
                        buf.clear();
                    }
                },
                x => buf.push(x)
            }

            if escape > 0 {
                escape -= 1;
            }
        }

        if !in_string {
            if in_comment {
                buf.clear();
                in_comment = false;
            } else if buf.trim() != "" {
                out.push(Token {kind: TokenType::Word, value: buf.to_string(), line_number: line_number});
            }
        }
        
        
        
        line_number += 1;
    }

    return Ok(out)
}