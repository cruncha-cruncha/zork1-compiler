use std::collections::{HashMap, HashSet};

use crate::{
    stats::{
        cross_ref::Populator,
        helpers::{get_nth_child_as_word, get_token_as_word},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use crate::stats::cross_ref::Codex;

use super::syntax::ILLEGAL;

pub struct SynonymStats<'a> {
    basis: HashMap<String, &'a ZilNode>,
    pub all_synonyms: HashSet<String>,
}

impl<'a> SynonymStats<'a> {
    pub fn new() -> SynonymStats<'a> {
        SynonymStats {
            basis: HashMap::new(),
            all_synonyms: HashSet::new(),
        }
    }

    pub fn get_basis(&self) -> &HashMap<String, &'a ZilNode> {
        return &self.basis;
    }
}

impl<'a> Populator<'a> for SynonymStats<'a> {
    fn add_node(&mut self, node: &'a ZilNode) {
        let name = get_nth_child_as_word(1, node);
        match name {
            Some(name) => {
                if self.basis.insert(name, node).is_some() {
                    panic!(
                        "Synonym node has duplicate name {}",
                        get_nth_child_as_word(1, node).unwrap()
                    );
                }
            }
            None => panic!("Synonym node has no name\n{}", format_file_location(&node)),
        }
    }

    fn crunch(&mut self) -> Result<(), String> {
        for (_, n) in self.basis.iter() {
            if n.children.len() < 3 {
                return Err(format!(
                    "Synonym node has less than three children\n{}",
                    format_file_location(n)
                ));
            }

            for c in n.children.iter().skip(1) {
                if c.node_type != ZilNodeType::Token(TokenType::Word) {
                    return Err(format!(
                        "Synonym node has non-word child\n{}",
                        format_file_location(&c)
                    ));
                }

                let val = get_token_as_word(c).unwrap();
                if ILLEGAL.is_match(&val) {
                    return Err(format!(
                        "Synonym has illegal char\n{}",
                        format_file_location(&c)
                    ));
                }

                if !self.all_synonyms.insert(val) {
                    return Err(format!(
                        "Synonym node has duplicate synonym {}\n{}",
                        get_token_as_word(c).unwrap(),
                        format_file_location(&c)
                    ));
                }
            }
        }

        Ok(())
    }

    fn validate(&self, _cross_ref: &crate::stats::cross_ref::CrossRef) -> Result<(), String> {
        Ok(())
    }
}

impl<'a> Codex for SynonymStats<'a> {
    fn lookup(&self, word: &str) -> Option<&ZilNode> {
        self.basis.get(word).map(|n| *n)
    }
}