use std::collections::HashMap;

use crate::{
    stats::{
        cross_ref::CrossRef,
        helpers::{get_token_as_word, num_children, parse_token_as_number, ValidationResult},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
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

    fn crunch(&mut self) -> ValidationResult<()> {
        let mut errors: Vec<String> = Vec::new();
        for (i, node) in self.basis.iter().enumerate() {
            match num_children(node, 3) {
                Ok(_) => {}
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            }

            let second_word = match get_token_as_word(&node.children[1]) {
                Ok(v) => v,
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            };

            if self.all_globals.contains_key(&second_word) {
                errors.push(format!(
                    "Global node defined twice:{}\n{}",
                    second_word,
                    format_file_location(&node)
                ));
                continue;
            }

            let third_child = &node.children[2];
            let val = match &third_child.node_type {
                ZilNodeType::Token(TokenType::Number) => {
                    parse_token_as_number(third_child).unwrap()
                }
                _ => {
                    errors.push(format!(
                        "Expected number or text, found {}\n{}",
                        third_child.node_type,
                        format_file_location(&third_child)
                    ));
                    continue;
                }
            };

            self.all_globals.insert(
                second_word.clone(),
                GlobalInfo {
                    index: i,
                    name: second_word,
                    val: val,
                },
            );
        }

        Ok(())
    }

    fn validate(&self, _cross_ref: &CrossRef) -> ValidationResult<()> {
        for key in self.all_globals.keys() {
            if CrossRef::name_is_illegal(key) {
                return Err(vec![format!("Illegal global name: {}", key)]);
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
    pub val: &'a i32,
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
                val: &info.val,
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
            val: &info.val,
        });
    }
}
