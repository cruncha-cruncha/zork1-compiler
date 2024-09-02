use std::collections::HashSet;

use crate::stats::cross_ref::Populator;
use crate::stats::helpers::get_token_as_word;
use crate::zil::file_table::format_file_location;
use crate::zil::node::ZilNode;

use crate::stats::cross_ref::Codex;

use super::syntax::ILLEGAL;

pub struct DirectionStats {
    basis: Option<ZilNode>,
    pub options: HashSet<String>,
}

impl DirectionStats {
    pub fn new() -> DirectionStats {
        DirectionStats {
            basis: None,
            options: HashSet::new(),
        }
    }
}

impl Populator for DirectionStats {
    fn add_node(&mut self, node: ZilNode) {
        if self.basis.is_none() {
            self.basis = Some(node);
        } else {
            panic!("DirectionCodex already has a basis");
        }
    }

    fn crunch(&mut self) -> Result<(), String> {
        if self.basis.is_none() {
            println!("WARNING: DirectionCodex has no basis. Please specify a <DIRECTIONS ... > cluster in one of input files.");
            return Ok(());
        }

        if self.basis.as_ref().unwrap().children.len() <= 1 {
            panic!("No directions");
        }

        for node in self.basis.as_ref().unwrap().children.iter().skip(1) {
            match get_token_as_word(node) {
                Some(name) => {
                    if ILLEGAL.is_match(&name) {
                        return Err(format!(
                            "Direction ({}) has illegal char\n{}",
                            &name,
                            format_file_location(&node)
                        ));
                    }

                    self.options.insert(name);
                }
                None => panic!("Direction node has non-word child"),
            }
        }

        Ok(())
    }

    fn validate(&self, _cross_ref: &crate::stats::cross_ref::CrossRef) -> Result<(), String> {
        Ok(())
    }
}

impl Codex for DirectionStats {
    fn lookup(&self, word: &str) -> Option<&ZilNode> {
        if self.options.contains(word) {
            return Some(self.basis.as_ref().unwrap());
        }

        None
    }
}
