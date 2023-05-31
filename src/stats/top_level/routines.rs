use std::collections::HashMap;

use crate::zil::{node::ZilNode, file_table::format_file_location};

use crate::stats::{cross_ref::Codex, helpers::get_nth_child_name};

pub struct RoutineCodex<'a> {
    basis: HashMap<String, &'a ZilNode>,
}

// <VERB? WORD1 WORD2 ...>
// corresponds to all syntax with = V-WORD1 or = V-WORD2 etc.

// <FSET? WORD1 WORD2>, <FCLEAR WORD1 WORD2>
// WORD1 is a room or object

// wtf is <QUEUE ...>

// <INTEGRAL-PART> is a ROUTINE

impl<'a> RoutineCodex<'a> {
    pub fn new() -> RoutineCodex<'a> {
        RoutineCodex {
            basis: HashMap::new(),
        }
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
            None => panic!("Routine node has no name\n{}", format_file_location(&node)),
        }
    }

    fn crunch(&mut self) -> Result<(), String> {
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
