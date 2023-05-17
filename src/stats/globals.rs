use std::collections::{HashMap};

use crate::zil::node::{ZilNode, ZilNodeType};

use super::{helpers::{get_nth_child_name}, top_level::Codex};

pub struct GlobalCodex<'a> {
    basis: HashMap<String, &'a ZilNode>,
}

impl<'a> GlobalCodex<'a> {
    pub fn new() -> GlobalCodex<'a> {
        GlobalCodex {
            basis: HashMap::new(),
        }
    }
}

impl<'a> IntoIterator for GlobalCodex<'a> {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.basis.keys().map(|k| k.clone()).collect::<Vec<String>>().into_iter()
    }
}

impl<'a> Codex<'a> for GlobalCodex<'a> {
    fn get_name(&self) -> String {
        String::from("globals")
    }

    fn add_node(&mut self, node: &'a ZilNode) {
        let name = get_nth_child_name(1, node);
        match name {
            Some(name) => {
                if self.basis.insert(name, node).is_some() {
                    panic!("Global node has duplicate name");
                }
            },
            None => panic!("Global node has no name"),
        }
    }

    fn crunch(&mut self) -> Result<(), String> {
        for n in self.basis.values() {
            if n.children.len() != 3 {
                return Err(String::from("Global node does not have exactly three children"));
            }

            match n.children[2].node_type {
                ZilNodeType::Cluster | ZilNodeType::TokenBunch(_) => (),
                _ => return Err(String::from("Global node has invalid third child type")),
            }
        }

        Ok(())
    }

    fn lookup(&self, word: &str) -> Option<&ZilNode> {
        self.basis.get(word).map(|n| *n)
    }
}