use std::collections::HashSet;

use crate::zil::node::ZilNode;

use super::{helpers::get_bunch_name, top_level::Codex};

pub struct DirectionCodex<'a> {
    basis: Option<&'a ZilNode>,
    pub options: HashSet<String>,
}

impl<'a> DirectionCodex<'a> {
    pub fn new() -> DirectionCodex<'a> {
        DirectionCodex {
            basis: None,
            options: HashSet::new(),
        }
    }
}

impl IntoIterator for DirectionCodex<'_> {
    type Item = String;
    type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.options.clone().into_iter()
    }
}

impl<'a> Codex<'a> for DirectionCodex<'a> {
    fn get_name(&self) -> String {
        String::from("directions")
    }

    fn add_node(&mut self, node: &'a ZilNode) {
        if self.basis.is_none() {
            self.basis = Some(node);
        } else {
            panic!("DirectionCodex already has a basis");
        }
    }

    fn crunch(&mut self) -> Result<(), String> {
        if self.basis.is_none() {
            return Err(String::from("DirectionCodex has no basis"));
        }

        if self.basis.unwrap().children.len() <= 1 {
            panic!("No directions");
        }

        for node in self.basis.unwrap().children.iter().skip(1) {
            match get_bunch_name(node) {
                Some(name) => {
                    self.options.insert(name);
                }
                None => panic!("Direction node has non-word child"),
            }
        }

        Ok(())
    }

    fn lookup(&self, word: &str) -> Option<&ZilNode> {
        if self.options.contains(word) {
            return Some(self.basis.unwrap());
        }

        None
    }
}
