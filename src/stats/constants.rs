use std::collections::HashMap;

use crate::zil::node::{ZilNode, ZilNodeType};

use super::{helpers::get_nth_child_name, top_level::Codex};

pub struct ConstantCodex<'a> {
    basis: HashMap<String, &'a ZilNode>,
}

impl<'a> ConstantCodex<'a> {
    pub fn new() -> ConstantCodex<'a> {
        ConstantCodex {
            basis: HashMap::new(),
        }
    }
}

impl<'a> IntoIterator for ConstantCodex<'a> {
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

impl<'a> Codex<'a> for ConstantCodex<'a> {
    fn get_name(&self) -> String {
        String::from("constants")
    }

    fn add_node(&mut self, node: &'a ZilNode) {
        let name = get_nth_child_name(1, node);
        match name {
            Some(name) => {
                if self.basis.insert(name, node).is_some() {
                    panic!("Constant node has duplicate name {}", get_nth_child_name(1, node).unwrap());
                }
            }
            None => panic!("Constant node has no name"),
        }
    }

    fn crunch(&mut self) -> Result<(), String> {
        for n in self.basis.values() {
            if n.children.len() != 3 {
                return Err(String::from(
                    "Constant node does not have exactly three children",
                ));
            }

            match n.children[2].node_type {
                ZilNodeType::Cluster => {
                    if n.children[2].children.len() > 0 {
                        return Err(String::from(
                            "Constant node has non-empty cluster as third child",
                        ));
                    }
                }
                ZilNodeType::TokenBunch(_) => (),
                _ => return Err(String::from("Constant node has invalid third child type")),
            }
        }

        Ok(())
    }

    fn lookup(&self, word: &str) -> Option<&ZilNode> {
        self.basis.get(word).map(|n| *n)
    }
}
