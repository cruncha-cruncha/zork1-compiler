use std::collections::HashSet;

use crate::stats::helpers::get_token_as_word;
use crate::zil::{file_table::format_file_location, node::ZilNode};

use crate::stats::cross_ref::{CrossRef, Populator};

pub struct BuzzStats {
    basis: Vec<ZilNode>,
    all: HashSet<String>,
}

impl BuzzStats {
    pub fn new() -> BuzzStats {
        BuzzStats {
            basis: Vec::new(),
            all: HashSet::new(),
        }
    }

    pub fn get_vals(&self) -> Vec<String> {
        self.all.iter().map(|s| String::from(s)).collect()
    }
}

impl Populator for BuzzStats {
    fn add_node(&mut self, node: ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> Result<(), String> {
        for n in self.basis.iter() {
            for c in n.children.iter().skip(1) {
                match get_token_as_word(c) {
                    Some(name) => {
                        if !self.all.insert(name) {
                            panic!(
                                "Buzz node has duplicate child word {}",
                                get_token_as_word(c).unwrap()
                            );
                        }
                    }
                    None => panic!("Buzz node has non-word child\n{}", format_file_location(&c)),
                }
            }
        }

        Ok(())
    }

    fn validate(&self, _cross_ref: &CrossRef) -> Result<(), String> {
        Ok(())
    }
}

pub struct BuzzIter<'a> {
    index: usize,
    all: &'a Vec<String>,
}

impl<'a> Iterator for BuzzIter<'a> {
    type Item = &'a String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.all.len() {
            None
        } else {
            self.index += 1;
            Some(&self.all[self.index - 1])
        }
    }
}
