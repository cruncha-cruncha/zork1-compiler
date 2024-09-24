use std::collections::HashMap;

use crate::{
    stats::{
        cross_ref::{Codex, Populator},
        helpers::{get_token_as_word, num_children_more_than, ValidationResult},
    },
    zil::{file_table::format_file_location, node::ZilNode},
};

pub struct SynonymStats {
    basis: Vec<ZilNode>,
    all_synonyms: HashMap<String, Vec<String>>,
}

impl SynonymStats {
    pub fn new() -> SynonymStats {
        SynonymStats {
            basis: Vec::new(),
            all_synonyms: HashMap::new(),
        }
    }
}

impl Codex<Vec<String>> for SynonymStats {
    fn lookup(&self, word: &str) -> Option<Vec<String>> {
        match self.all_synonyms.get(word) {
            Some(line) => Some(line.iter().map(|s| String::from(s)).collect()),
            None => None,
        }
    }
}

impl Populator for SynonymStats {
    fn add_node(&mut self, node: ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> ValidationResult<()> {
        let mut errors: Vec<String> = Vec::new();
        for line in self.basis.iter() {
            let mut synonym_errors: Vec<String> = Vec::new();
            let mut words: Vec<String> = Vec::new();

            match num_children_more_than(line, 2) {
                Ok(_) => {}
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            }

            let second_word = match get_token_as_word(&line.children[1]) {
                Ok(v) => Some(v),
                Err(e) => {
                    synonym_errors.push(e);
                    None
                }
            };

            for c in line.children.iter().skip(2) {
                let word = match get_token_as_word(c) {
                    Ok(v) => v,
                    Err(e) => {
                        synonym_errors.push(e);
                        continue;
                    }
                };

                words.push(word);
            }

            if second_word.is_some() {
                let second_word = second_word.as_ref().unwrap();
                if self.all_synonyms.contains_key(second_word) {
                    errors.push(format!(
                        "Synonym node is not unique: already have synonyms for {}\n{}",
                        second_word,
                        format_file_location(&line)
                    ));
                }
            }

            if synonym_errors.len() > 0 {
                errors.append(&mut synonym_errors);
                continue;
            }

            self.all_synonyms.insert(second_word.unwrap(), words);
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(())
    }

    fn validate(&self, _cross_ref: &crate::stats::cross_ref::CrossRef) -> ValidationResult<()> {
        Ok(())
    }
}
