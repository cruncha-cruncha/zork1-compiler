use std::collections::HashMap;

use crate::{
    stats::{
        cross_ref::CrossRef,
        helpers::{get_token_as_number, get_token_as_word},
    },
    zil::{file_table::format_file_location, node::ZilNode},
};

use crate::stats::{cross_ref::Codex, cross_ref::Populator};

pub struct GlobalStats {
    basis: Vec<ZilNode>,
    all_globals: HashMap<String, GlobalInfo>,
}

pub struct GlobalInfo {
    #[allow(dead_code)]
    index: usize,
    name: String,
    val: i32,
}

impl GlobalStats {
    pub fn new() -> GlobalStats {
        GlobalStats {
            basis: Vec::new(),
            all_globals: HashMap::new(),
        }
    }

    pub fn as_codex(&self) -> GlobalCodex {
        GlobalCodex {
            index: 0,
            basis: &self.basis,
            all_objects: &self.all_globals,
        }
    }
}

impl Populator for GlobalStats {
    fn add_node(&mut self, node: ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> Result<(), String> {
        for (i, node) in self.basis.iter().enumerate() {
            if node.children.len() != 3 {
                return Err(format!(
                    "Possible global node doesn't have three children\n{}",
                    format_file_location(&node)
                ));
            }

            let first_word = get_token_as_word(&node.children[0]).unwrap_or_default();
            if first_word != "GLOBAL" {
                unreachable!();
            }

            let second_word = get_token_as_word(&node.children[1]);
            if second_word.is_none() {
                return Err(format!(
                    "Global node has non-word second child\n{}",
                    format_file_location(&node)
                ));
            }

            let second_word = second_word.unwrap();
            if self.all_globals.contains_key(&second_word) {
                return Err(format!(
                    "Global node defined twice:{}\n{}",
                    second_word,
                    format_file_location(&node)
                ));
            }

            let number = get_token_as_number(&node.children[2]);
            if number.is_none() {
                return Err(format!(
                    "Global node has non-number third child\n{}",
                    format_file_location(&node)
                ));
            }

            self.all_globals.insert(
                second_word.clone(),
                GlobalInfo {
                    index: i,
                    name: second_word,
                    val: number.unwrap(),
                },
            );
        }

        Ok(())
    }

    fn validate(&self, _cross_ref: &CrossRef) -> Result<(), String> {
        for key in self.all_globals.keys() {
            if CrossRef::name_is_illegal(key) {
                return Err(format!("Illegal global name: {}", key));
            }
        }

        Ok(())
    }
}

pub struct GlobalCodex<'a> {
    index: usize,
    basis: &'a Vec<ZilNode>,
    all_objects: &'a HashMap<String, GlobalInfo>,
}

pub struct GlobalCodexValue<'a> {
    pub name: &'a String,
    pub val: i32,
}

impl<'a> Iterator for GlobalCodex<'a> {
    type Item = GlobalCodexValue<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.basis.len() {
            None
        } else {
            self.index += 1;
            let node = &self.basis[self.index - 1];
            let key = get_token_as_word(&node.children[1]).unwrap();
            let info = self.all_objects.get(&key).unwrap();

            Some(GlobalCodexValue {
                name: &info.name,
                val: info.val,
            })
        }
    }
}

impl<'a> Codex<GlobalCodexValue<'a>> for GlobalCodex<'a> {
    fn lookup(&self, word: &str) -> Option<GlobalCodexValue<'a>> {
        let info = self.all_objects.get(word);

        if info.is_none() {
            return None;
        }

        let info = info.unwrap();

        return Some(GlobalCodexValue {
            name: &info.name,
            val: info.val,
        });
    }
}
