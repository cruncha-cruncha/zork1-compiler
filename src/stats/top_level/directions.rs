use std::collections::HashSet;

use crate::stats::cross_ref::{CrossRef, Populator};
use crate::stats::helpers::get_token_as_word;
use crate::zil::file_table::format_file_location;
use crate::zil::node::ZilNode;

use crate::stats::cross_ref::Codex;

pub struct DirectionStats {
    basis: Vec<ZilNode>,
    all: HashSet<String>,
}

impl DirectionStats {
    pub fn new() -> DirectionStats {
        DirectionStats {
            basis: Vec::new(),
            all: HashSet::new(),
        }
    }

    pub fn get_vals(&self) -> Vec<String> {
        self.all.iter().map(|s| String::from(s)).collect()
    }
}

impl Populator for DirectionStats {
    fn add_node(&mut self, node: ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> Result<(), String> {
        if self.basis.len() == 0 {
            println!(
                "WARNING: DirectionCodex has no basis. Please provide a <DIRECTIONS ... > cluster."
            );
            return Ok(());
        }

        for node in self.basis.iter() {
            if node.children.len() < 2 {
                return Err(format!(
                    "Possible directions node doesn't have enough children\n{}",
                    format_file_location(&node)
                ));
            }

            let first_word: String = get_token_as_word(&node.children[0]).unwrap_or_default();
            if first_word != "DIRECTIONS" {
                unreachable!();
            }

            for c in node.children.iter().skip(1) {
                let word = get_token_as_word(c);
                if word.is_none() {
                    return Err(format!(
                        "Directions node has non-word child\n{}",
                        format_file_location(&c)
                    ));
                }

                let word = word.unwrap();
                if !word.chars().all(char::is_alphabetic) {
                    return Err(format!(
                        "Directions node has non-alphabetic child\n{}",
                        format_file_location(&c)
                    ));
                }

                self.all.insert(word);
            }
        }

        Ok(())
    }

    fn validate(&self, _cross_ref: &CrossRef) -> Result<(), String> {
        Ok(())
    }
}

impl Codex<String> for DirectionStats {
    fn lookup(&self, word: &str) -> Option<String> {
        match self.all.get(word) {
            Some(val) => Some(String::from(val)),
            None => None,
        }
    }
}
