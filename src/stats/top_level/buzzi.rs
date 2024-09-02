use std::collections::HashSet;

use crate::stats::helpers::get_token_as_word;
use crate::zil::{file_table::format_file_location, node::ZilNode};

use crate::stats::cross_ref::Populator;

pub struct BuzzStats {
    basis: Vec<ZilNode>,
    pub all: HashSet<String>,
}

impl BuzzStats {
    pub fn new() -> BuzzStats {
        BuzzStats {
            basis: Vec::new(),
            all: HashSet::new(),
        }
    }

    pub fn validate_object_synonyms(
        &self,
        object_synonyms: &HashSet<String>,
    ) -> Result<(), String> {
        for s in self.all.iter() {
            if object_synonyms.contains(s) {
                return Err(format!("BUZZ has object synonym {}", s));
            }
        }

        Ok(())
    }

    pub fn validate_prepositions(&self, prepositions: &HashSet<String>) -> Result<(), String> {
        for s in self.all.iter() {
            if prepositions.contains(s) {
                return Err(format!("BUZZ has preposition {}", s));
            }
        }

        Ok(())
    }

    pub fn validate_verbs(&self, verbs: &HashSet<String>) -> Result<(), String> {
        for s in self.all.iter() {
            if verbs.contains(s) {
                return Err(format!("BUZZ has verb {}", s));
            }
        }

        Ok(())
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

    fn validate(&self, cross_ref: &crate::stats::cross_ref::CrossRef) -> Result<(), String> {
        self.validate_object_synonyms(&cross_ref.objects.groups.synonyms)?;
        self.validate_prepositions(&cross_ref.syntax.prepositions)?;
        self.validate_verbs(&cross_ref.syntax.firsts)?;

        Ok(())
    }
}
