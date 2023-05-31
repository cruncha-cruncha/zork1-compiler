use std::collections::HashMap;

use crate::zil::{
    file_table::format_file_location,
    node::{ZilNode, ZilNodeType},
};

use crate::stats::{cross_ref::Codex, helpers::get_nth_child_name};

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

impl<'a> Codex<'a> for GlobalCodex<'a> {
    fn get_name(&self) -> String {
        String::from("globals")
    }

    fn add_node(&mut self, node: &'a ZilNode) {
        let name = get_nth_child_name(1, node);
        match name {
            Some(name) => {
                if self.basis.insert(name, node).is_some() {
                    panic!(
                        "Global node has duplicate name {}",
                        get_nth_child_name(1, node).unwrap()
                    );
                }
            }
            None => panic!("Global node has no name\n{}", format_file_location(&node)),
        }
    }

    fn crunch(&mut self) -> Result<(), String> {
        for n in self.basis.values() {
            if n.children.len() != 3 {
                return Err(format!(
                    "Global node does not have exactly three children\n{}",
                    format_file_location(n)
                ));
            }

            match n.children[2].node_type {
                ZilNodeType::Cluster | ZilNodeType::TokenBunch(_) => (),
                _ => {
                    return Err(format!(
                        "Global node has invalid third child type\n{}",
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
