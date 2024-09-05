use std::collections::HashMap;

use crate::{
    stats::helpers::{get_token_as_number, get_token_as_text, get_token_as_word},
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
    index: usize,
    var: Var,
}

impl GlobalInfo {
    pub fn new() -> GlobalInfo {
        GlobalInfo {
            index: 0,
            var: Var::new(),
        }
    }
}

pub struct Var {
    pub name: String,
    pub val: Option<VarVal>,
}

impl Var {
    pub fn new() -> Var {
        Var {
            name: String::new(),
            val: None,
        }
    }
}

pub enum VarVal {
    List(Vec<VarVal>),
    Single(VarValSingle),
}

impl VarVal {
    pub fn parse(node: &ZilNode) -> Result<VarVal, ()> {
        match node.node_type {
            ZilNodeType::Token(_) => match VarValSingle::parse(node) {
                Ok(single) => Ok(VarVal::Single(single)),
                Err(e) => Err(e),
            },
            ZilNodeType::Cluster => {
                let mut contents: Vec<VarVal> = Vec::new();
                for c in node.children.iter() {
                    match VarVal::parse(c) {
                        Ok(val) => contents.push(val),
                        Err(e) => return Err(e),
                    }
                }

                Ok(VarVal::List(contents))
            }
            _ => Err(()),
        }
    }
}

pub enum VarValSingle {
    Text(String),
    Word(String),
    Number(i32),
}

impl VarValSingle {
    pub fn parse(node: &ZilNode) -> Result<VarValSingle, ()> {
        match node.node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&node.children[2]).unwrap();
                Ok(VarValSingle::Word(word))
            }
            ZilNodeType::Token(TokenType::Text) => {
                let text = get_token_as_text(&node.children[2]).unwrap();
                Ok(VarValSingle::Text(text))
            }
            ZilNodeType::Token(TokenType::Number) => {
                let number = get_token_as_number(&node.children[2]).unwrap();
                Ok(VarValSingle::Number(number))
            }
            _ => Err(()),
        }
    }
}

impl GlobalStats {
    pub fn new() -> GlobalStats {
        GlobalStats {
            basis: Vec::new(),
            all_globals: HashMap::new(),
        }
    }
}

impl Populator for GlobalStats {
    fn add_node(&mut self, node: ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> Result<(), String> {
        for (i, node) in self.basis.iter().enumerate() {
            let mut info = GlobalInfo::new();

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

            info.index = i;

            info.var.name = second_word.clone();

            info.var.val = match VarVal::parse(&node.children[1]) {
                Ok(val) => Some(val),
                Err(_) => {
                    return Err(format!(
                        "Global node has bad value\n{}",
                        format_file_location(&node.children[i])
                    ))
                }
            };

            self.all_globals.insert(second_word, info);
        }

        Ok(())
    }

    fn validate(&self, _cross_ref: &crate::stats::cross_ref::CrossRef) -> Result<(), String> {
        // TODO
        // ???

        Ok(())
    }
}

pub struct GlobalCodex<'a> {
    index: usize,
    basis: &'a Vec<ZilNode>,
    all_objects: &'a HashMap<String, GlobalInfo>,
}

impl<'a> Iterator for GlobalCodex<'a> {
    type Item = &'a Var;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.basis.len() {
            None
        } else {
            self.index += 1;
            let node = &self.basis[self.index - 1];
            let key = get_token_as_word(&node.children[1]).unwrap();
            let info = self.all_objects.get(&key).unwrap();

            Some(&info.var)
        }
    }
}

impl<'a> Codex<&'a Var> for GlobalCodex<'a> {
    fn lookup(&self, word: &str) -> Option<&'a Var> {
        let info = self.all_objects.get(word);

        if info.is_none() {
            return None;
        }

        let info = info.unwrap();

        return Some(&info.var);
    }
}
