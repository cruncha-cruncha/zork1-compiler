use std::collections::HashMap;

use crate::zil::node::ZilNode;

use super::{helpers::get_nth_child_name, top_level::Codex};

pub struct SynonymCodex<'a> {
    basis: HashMap<String, &'a ZilNode>,
}

impl<'a> SynonymCodex<'a> {
    pub fn new() -> SynonymCodex<'a> {
        SynonymCodex {
            basis: HashMap::new(),
        }
    }
}

impl<'a> IntoIterator for SynonymCodex<'a> {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.basis
            .keys()
            .map(|k| k.clone())
            .collect::<Vec<String>>()
            .into_iter()
    }
}

impl<'a> Codex<'a> for SynonymCodex<'a> {
    fn get_name(&self) -> String {
        String::from("synonyms")
    }

    fn add_node(&mut self, node: &'a ZilNode) {
        let name = get_nth_child_name(1, node);
        match name {
            Some(name) => {
                if self.basis.insert(name, node).is_some() {
                    panic!("Synonym node has duplicate name {}", get_nth_child_name(1, node).unwrap());
                }
            }
            None => panic!("Synonym node has no name"),
        }
    }

    fn crunch(&mut self) -> Result<(), String> {
        // TODO
        Ok(())
    }

    fn lookup(&self, word: &str) -> Option<&ZilNode> {
        self.basis.get(word).map(|n| *n)
    }
}
