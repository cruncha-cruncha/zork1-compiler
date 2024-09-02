use std::collections::HashMap;

use crate::{
    stats::{cross_ref::Populator, helpers::get_nth_child_as_word},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use crate::stats::cross_ref::Codex;

pub struct ConstantStats {
    basis: HashMap<String, ZilNode>,
    pub values: HashMap<String, i32>,
}

impl ConstantStats {
    pub fn new() -> ConstantStats {
        ConstantStats {
            basis: HashMap::new(),
            values: HashMap::new(),
        }
    }
}

impl Populator for ConstantStats {
    fn add_node(&mut self, node: ZilNode) {
        let name = get_nth_child_as_word(1, &node);
        match name {
            Some(name) => {
                if self.basis.insert(name.clone(), node).is_some() {
                    panic!("Constant node has duplicate name {}", name);
                }
            }
            None => panic!("Constant node has no name\n{}", format_file_location(&node)),
        }
    }

    fn crunch(&mut self) -> Result<(), String> {
        for (k, n) in self.basis.iter() {
            if n.children.len() != 3 {
                return Err(format!(
                    "Constant node does not have exactly three children\n{}",
                    format_file_location(n)
                ));
            }

            match n.children[1].node_type {
                ZilNodeType::Token(TokenType::Word) => (),
                _ => {
                    return Err(format!(
                        "Constant node has invalid second child type\n{}",
                        format_file_location(n)
                    ))
                }
            }

            match n.children[2].node_type {
                ZilNodeType::Token(TokenType::Number) => {
                    let value = n.children[2]
                        .get_first_token()
                        .unwrap()
                        .value
                        .parse::<i32>();
                    match value {
                        Ok(value) => {
                            self.values.insert(k.clone(), value);
                        }
                        Err(_) => {
                            return Err(format!(
                                "Constant node has invalid third child value\n{}",
                                format_file_location(n)
                            ))
                        }
                    }
                }
                ZilNodeType::Cluster => {
                    if n.children[2].children.len() != 0 {
                        return Err(format!(
                            "Constant node has invalid third child type\n{}",
                            format_file_location(n)
                        ));
                    } else {
                        // 0 is not exactly null, but it might have to do
                        self.values.insert(k.clone(), 0);
                    }
                }
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

    fn validate(&self, _cross_ref: &crate::stats::cross_ref::CrossRef) -> Result<(), String> {
        Ok(())
    }
}

impl Codex for ConstantStats {
    fn lookup(&self, word: &str) -> Option<&ZilNode> {
        self.basis.get(word)
    }
}
