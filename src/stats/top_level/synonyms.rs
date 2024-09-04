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

pub struct SynonymStats {
    basis: HashMap<String, ZilNode>,
    pub all_synonyms: HashMap<String, Vec<String>>,
}

impl SynonymStats {
    pub fn new() -> SynonymStats {
        SynonymStats {
            basis: HashMap::new(),
            all_synonyms: HashMap::new(),
        }
    }

    pub fn get_basis(&self) -> &HashMap<String, ZilNode> {
        return &self.basis;
    }
}

impl Populator for SynonymStats {
    fn add_node(&mut self, node: ZilNode) {
        let name = get_nth_child_as_word(1, &node);
        match name {
            Some(name) => {
                if self.basis.insert(name.clone(), node).is_some() {
                    panic!("Synonym node has duplicate name {}", name);
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
            }

            let substitutes: Vec<String> = n
                .children
                .iter()
                .skip(2)
                .map(|c| get_token_as_word(c).unwrap())
                .collect();

            match self
                .all_synonyms
                .insert(get_token_as_word(&n.children[1]).unwrap(), substitutes)
            {
                Some(_) => {
                    return Err(format!(
                        "Synonym node has duplicate synonym {}\n{}",
                        get_token_as_word(&n.children[1]).unwrap(),
                        format_file_location(&n)
                    ));
                }
                None => (),
            }
        }

        Ok(())
    }

    fn validate(&self, _cross_ref: &crate::stats::cross_ref::CrossRef) -> Result<(), String> {
        Ok(())
    }
}

impl Codex for SynonymStats {
    fn lookup(&self, word: &str) -> Option<&ZilNode> {
        self.basis.get(word)
    }
}
