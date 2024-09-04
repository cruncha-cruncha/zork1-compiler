use std::collections::HashMap;

use crate::{
    stats::helpers::{get_nth_child_as_word, get_token_as_number, get_token_as_word},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use crate::stats::{cross_ref::Codex, cross_ref::Populator};

pub struct GlobalStats {
    basis: HashMap<String, ZilNode>,
    pub info: HashMap<String, GlobalValType>,
}

pub enum GlobalValType {
    Empty,
    Table(bool, Vec<GlobalValType>), // bool == LTABLE
    Text(String),
    Word(String),
    Number(i32),
}

impl GlobalStats {
    pub fn new() -> GlobalStats {
        GlobalStats {
            basis: HashMap::new(),
            info: HashMap::new(),
        }
    }

    fn validate_nested_cluster(&self, node: &ZilNode) -> Result<GlobalValType, String> {
        if node.children.len() < 1 {
            return Ok(GlobalValType::Empty);
        }

        let name = match get_nth_child_as_word(0, &node) {
            Some(name) => name,
            None => {
                return Err(format!(
                    "Global node has no a bad child cluster\n{}",
                    format_file_location(&node)
                ));
            }
        };

        if name != "TABLE" && name != "LTABLE" && name != "ITABLE" {
            return Err(format!(
                "Global node has a bad child cluster\n{}",
                format_file_location(&node)
            ));
        }

        let mut skip = 1;
        if node.children.len() > 1 && node.children[1].node_type == ZilNodeType::Group {
            let group = &node.children[1];
            if group.children.len() != 1 {
                return Err(format!(
                    "Global node has a bad child group (v1) in cluster\n{}",
                    format_file_location(&node)
                ));
            }

            let name = match get_nth_child_as_word(0, group) {
                Some(name) => name,
                None => {
                    return Err(format!(
                        "Global node has a bad child group (v2) in cluster\n{}",
                        format_file_location(&node)
                    ));
                }
            };

            if name != "PURE" {
                return Err(format!(
                    "Global node has a bad child group (v3) in cluster\n{}",
                    format_file_location(&node)
                ));
            }

            skip = 2;
        }

        let mut elems: Vec<GlobalValType> = Vec::new();

        for n in node.children.iter().skip(skip) {
            match n.node_type {
                ZilNodeType::Cluster => match self.validate_nested_cluster(n) {
                    Ok(v) => elems.push(v),
                    Err(e) => return Err(e),
                },
                ZilNodeType::Token(TokenType::Text) => {
                    elems.push(GlobalValType::Text(
                        n.get_first_token().unwrap().value.clone(),
                    ));
                }
                ZilNodeType::Token(TokenType::Word) => {
                    elems.push(GlobalValType::Word(get_token_as_word(n).unwrap()));
                }
                ZilNodeType::Token(TokenType::Number) => {
                    elems.push(GlobalValType::Number(get_token_as_number(n).unwrap()));
                }
                _ => {
                    return Err(format!(
                        "Global node has a bad child cluster node type\n{}",
                        format_file_location(&node)
                    ));
                }
            }
        }

        Ok(GlobalValType::Table(name == "LTABLE", elems))
    }
}

// https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter
// type Item = T; // consumes self
// type Item = &'a T; // immutable reference
// type Item = &'a mut T; // mutable reference

// impl<'a> IntoIterator for GlobalCodex<'a> {
//     type Item = String; // &'b str is possible, but the extra lifetime is a hassle
//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.basis
//             .keys()
//             .map(|k| k.clone())
//             .collect::<Vec<String>>()
//             .into_iter()
//     }
// }

impl Populator for GlobalStats {
    fn add_node(&mut self, node: ZilNode) {
        let name = get_nth_child_as_word(1, &node);
        match name {
            Some(name) => {
                if self.basis.insert(name.clone(), node).is_some() {
                    panic!("Global node has duplicate name {}", name);
                }
            }
            None => panic!("Global node has no name\n{}", format_file_location(&node)),
        }
    }

    fn crunch(&mut self) -> Result<(), String> {
        for (k, n) in self.basis.iter() {
            if n.children.len() != 3 {
                return Err(format!(
                    "Global node does not have exactly three children\n{}",
                    format_file_location(n)
                ));
            }

            match n.children[2].node_type {
                ZilNodeType::Token(TokenType::Text) => {
                    let val = n.children[2].get_first_token().unwrap().value.clone();
                    self.info.insert(k.clone(), GlobalValType::Text(val));
                }
                ZilNodeType::Token(TokenType::Word) => {
                    let val = get_token_as_word(&n.children[2]).unwrap();
                    self.info.insert(k.clone(), GlobalValType::Word(val));
                }
                ZilNodeType::Token(TokenType::Number) => {
                    let val = get_token_as_number(&n.children[2]).unwrap();
                    self.info.insert(k.clone(), GlobalValType::Number(val));
                }
                ZilNodeType::Cluster => match self.validate_nested_cluster(&n.children[2]) {
                    Ok(v) => {
                        self.info.insert(k.clone(), v);
                    }
                    Err(e) => return Err(e),
                },
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

    fn validate(&self, _cross_ref: &crate::stats::cross_ref::CrossRef) -> Result<(), String> {
        Ok(())
    }
}

impl Codex for GlobalStats {
    fn lookup(&self, word: &str) -> Option<&ZilNode> {
        self.basis.get(word)
    }
}
