use std::collections::HashMap;

use crate::zil::{
    file_table::format_file_location,
    node::{ZilNode, ZilNodeType},
};

use crate::stats::{cross_ref::Codex, helpers::get_nth_child_name};

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

impl<'a> Codex<'a> for ConstantCodex<'a> {
    fn get_name(&self) -> String {
        String::from("constants")
    }

    fn add_node(&mut self, node: &'a ZilNode) {
        let name = get_nth_child_name(1, node);
        match name {
            Some(name) => {
                if self.basis.insert(name, node).is_some() {
                    panic!(
                        "Constant node has duplicate name {}",
                        get_nth_child_name(1, node).unwrap()
                    );
                }
            }
            None => panic!("Constant node has no name\n{}", format_file_location(&node)),
        }
    }

    fn crunch(&mut self) -> Result<(), String> {
        for n in self.basis.values() {
            if n.children.len() != 3 {
                return Err(format!(
                    "Constant node does not have exactly three children\n{}",
                    format_file_location(n)
                ));
            }

            match n.children[2].node_type {
                ZilNodeType::Cluster => {
                    if n.children[2].children.len() > 0 {
                        return Err(format!(
                            "Constant node has non-empty cluster as third child\n{}",
                            format_file_location(n)
                        ));
                    }
                }
                ZilNodeType::TokenBunch(_) => (),
                _ => {
                    return Err(format!(
                        "Constant node has invalid third child type\n{}",
                        format_file_location(n)
                    ))
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
