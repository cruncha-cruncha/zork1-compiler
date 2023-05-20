use std::collections::HashMap;

use crate::zil::node::ZilNode;

use super::{helpers::get_nth_child_name, top_level::Codex};

pub struct RoutineCodex<'a> {
    basis: HashMap<String, &'a ZilNode>,
}

impl<'a> RoutineCodex<'a> {
    pub fn new() -> RoutineCodex<'a> {
        RoutineCodex {
            basis: HashMap::new(),
        }
    }
}

impl<'a> IntoIterator for RoutineCodex<'a> {
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

impl<'a> Codex<'a> for RoutineCodex<'a> {
    fn get_name(&self) -> String {
        String::from("routines")
    }

    fn add_node(&mut self, node: &'a ZilNode) {
        let name = get_nth_child_name(1, node);
        match name {
            Some(name) => {
                if self.basis.insert(name, node).is_some() {
                    panic!("Routine node has duplicate name {}", get_nth_child_name(1, node).unwrap());
                }
            }
            None => panic!("Routine node has no name"),
        }
    }

    fn crunch(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn lookup(&self, word: &str) -> Option<&ZilNode> {
        self.basis.get(word).map(|n| *n)
    }
}
