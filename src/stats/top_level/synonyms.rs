use std::collections::HashMap;

use crate::{
    stats::{
        cross_ref::{Codex, Populator},
        helpers::get_token_as_word,
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

    fn crunch(&mut self) -> Result<(), String> {
        for line in self.basis.iter() {
            let mut words: Vec<String> = Vec::new();

            if line.children.len() < 3 {
                return Err(format!(
                    "Possible synonym node has less than three children\n{}",
                    format_file_location(line)
                ));
            }

            let first_word = get_token_as_word(&line.children[0]).unwrap_or_default();
            if first_word != "SYNONYM" {
                unreachable!();
            }

            let second_word = get_token_as_word(&line.children[1]);
            if second_word.is_none() {
                return Err(format!(
                    "Synonym node has non-word second child\n{}",
                    format_file_location(&line)
                ));
            }

            let second_word = second_word.unwrap();
            if self.all_synonyms.contains_key(&second_word) {
                return Err(format!(
                    "Synonym node is not unique: already have synonyms for {}\n{}",
                    second_word,
                    format_file_location(&line)
                ));
            }

            for c in line.children.iter().skip(2) {
                let word = get_token_as_word(c);

                if word.is_none() {
                    return Err(format!(
                        "Synonym node has non-word child\n{}",
                        format_file_location(&c)
                    ));
                }

                words.push(word.unwrap());
            }

            self.all_synonyms.insert(first_word, words);
        }

        Ok(())
    }

    fn validate(&self, _cross_ref: &crate::stats::cross_ref::CrossRef) -> Result<(), String> {
        Ok(())
    }
}
