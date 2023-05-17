use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use zil::file_table::FileKey;

use super::file_table::get_BufReader;
use super::file_table::FileTable;
use super::file_table::FileTableLocation;

pub trait CharGen: Iterator<Item = Result<CharInfo, io::Error>> + FileTableLocation {}

pub struct CharGenerator<'a> {
    files: &'a mut FileTable,
    reader: Option<BufReader<File>>,
    char_buf: Vec<char>,
    buf_index: usize,
    file_key: FileKey,
    line_number: u64,
}

pub struct CharInfo {
    pub val: char,
    pub file_key: FileKey,
    pub line_number: u64,
    pub char_number: u64,
}

impl<'a> CharGenerator<'a> {
    fn get_next_reader(&mut self) -> Option<BufReader<File>> {
        let mut out: Option<BufReader<File>> = None;

        while out.is_none() {
            let (file_key, file_path) = match self.files.next() {
                Some(v) => v,
                None => return None,
            };

            out = get_BufReader(&file_path);

            if !out.is_none() {
                self.file_key = file_key;
                self.line_number = 0;
                break;
            }
        }

        out
    }
}

impl<'a> FileTableLocation for CharGenerator<'a> {
    fn get_file_key(&self) -> FileKey {
        self.file_key
    }

    fn get_line_number(&self) -> u64 {
        self.line_number
    }

    fn get_char_number(&self) -> u64 {
        self.buf_index as u64
    }
}

impl<'a> Iterator for CharGenerator<'a> {
    type Item = Result<CharInfo, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf_index >= self.char_buf.len() {
            self.char_buf.clear();
        }

        if self.reader.is_none() {
            self.reader = self.get_next_reader();
            if self.reader.is_none() {
                return None;
            }
        }

        while self.char_buf.len() <= 0 {
            let mut line_buf = String::new();
            match self.reader.as_mut().unwrap().read_line(&mut line_buf) {
                Err(e) => return Some(Err(e)),
                Ok(0) => {
                    self.reader = self.get_next_reader();
                    if self.reader.is_none() {
                        return None;
                    }
                    continue;
                }
                Ok(_) => (),
            };

            self.buf_index = 0;
            self.line_number += 1;
            self.char_buf.extend(line_buf.trim().chars());
            self.char_buf.push('\n');
        }

        let out = CharInfo {
            val: self.char_buf[self.buf_index],
            file_key: self.file_key,
            line_number: self.line_number,
            char_number: self.buf_index as u64,
        };
        self.buf_index += 1;
        Some(Ok(out))
    }
}

impl<'a> CharGen for CharGenerator<'a> {}

pub fn new<'a>(files: &'a mut FileTable) -> CharGenerator<'a> {
    CharGenerator {
        files,
        reader: None,
        char_buf: Vec::new(),
        buf_index: 0,
        file_key: 0,
        line_number: 0,
    }
}
