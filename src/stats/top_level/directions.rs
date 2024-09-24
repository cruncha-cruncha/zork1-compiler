use std::collections::BTreeSet;

use crate::stats::cross_ref::{CrossRef, Populator};
use crate::stats::helpers::{get_token_as_word, num_children_more_than, ValidationResult};
use crate::zil::node::ZilNode;

use crate::stats::cross_ref::Codex;

pub struct DirectionStats {
    basis: Vec<ZilNode>,
    all: BTreeSet<String>,
}

impl DirectionStats {
    pub fn new() -> DirectionStats {
        DirectionStats {
            basis: Vec::new(),
            all: BTreeSet::new(),
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

    fn crunch(&mut self) -> ValidationResult<()> {
        if self.basis.len() == 0 {
            println!(
                "WARNING: DirectionCodex has no basis. Please provide a <DIRECTIONS ... > cluster."
            );
            return Ok(());
        }

        for node in self.basis.iter() {
            match num_children_more_than(node, 1) {
                Ok(_) => (),
                Err(e) => return Err(vec![e]),
            };

            for c in node.children.iter().skip(1) {
                let word = match get_token_as_word(c) {
                    Ok(v) => v,
                    Err(e) => return Err(vec![e]),
                };

                self.all.insert(word);
            }
        }

        Ok(())
    }

    fn validate(&self, _cross_ref: &CrossRef) -> ValidationResult<()> {
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
