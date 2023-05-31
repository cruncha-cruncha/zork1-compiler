use std::collections::HashMap;

use crate::zil::{
    file_table::format_file_location,
    node::{TokenBunchType, ZilNode, ZilNodeType},
};

use crate::stats::{cross_ref::Codex, helpers::get_nth_child_name};

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

impl<'a> Codex<'a> for SynonymCodex<'a> {
    fn get_name(&self) -> String {
        String::from("synonyms")
    }

    fn add_node(&mut self, node: &'a ZilNode) {
        let name = get_nth_child_name(1, node);
        match name {
            Some(name) => {
                if self.basis.insert(name, node).is_some() {
                    panic!(
                        "Synonym node has duplicate name {}",
                        get_nth_child_name(1, node).unwrap()
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
                if c.node_type != ZilNodeType::TokenBunch(TokenBunchType::Word) {
                    return Err(format!(
                        "Synonym node has non-word child\n{}",
                        format_file_location(&c)
                    ));
                }
            }
        }

        Ok(())
    }

    fn lookup(&self, word: &str) -> Option<&ZilNode> {
        self.basis.get(word).map(|n| *n)
    }

    fn into_iter(&self) -> std::vec::IntoIter<String> {
        self.basis
            .keys()
            .map(|k| k.clone())
            .collect::<Vec<String>>()
            .into_iter()
    }
}
