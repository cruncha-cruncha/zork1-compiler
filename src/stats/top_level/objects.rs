use std::collections::{HashMap, HashSet};

use crate::{
    stats::{
        cross_ref::{Codex, Populator},
        helpers::{get_token_as_text, get_token_as_word},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use super::globals::VarVal;

pub struct ObjectStats {
    basis: Vec<ZilNode>,
    all_objects: HashMap<String, ObjectInfo>,
}

pub struct ObjectInfo {
    index: usize,
    name: String,
    synonyms: HashSet<String>,
    loc: Option<String>, // can be room or object
    desc: Option<String>,
    vars: HashMap<String, VarVal>,
}

impl ObjectInfo {
    pub fn new() -> ObjectInfo {
        ObjectInfo {
            index: 0,
            name: String::new(),
            synonyms: HashSet::new(),
            loc: None,
            desc: None,
            vars: HashMap::new(),
        }
    }

    fn crunch_synonyms(node: &ZilNode) -> Result<HashSet<String>, String> {
        if node.children.len() < 2 {
            return Err(format!(
                "Synonyms group has too few children\n{}",
                format_file_location(&node)
            ));
        }

        let mut out: HashSet<String> = HashSet::new();
        for c in node.children.iter().skip(1) {
            let word = get_token_as_word(c);
            if word.is_none() {
                return Err(format!(
                    "Synonyms group has non-word child\n{}",
                    format_file_location(&c)
                ));
            }

            out.insert(word.unwrap());
        }

        Ok(out)
    }

    fn crunch_location(node: &ZilNode) -> Result<String, String> {
        if node.children.len() != 2 {
            return Err(format!(
                "Location node doesn't have 2 children\n{}",
                format_file_location(&node)
            ));
        }

        let word = get_token_as_word(&node.children[1]);
        if word.is_none() {
            return Err(format!(
                "Location node has non-word second child\n{}",
                format_file_location(&node.children[1])
            ));
        }

        Ok(word.unwrap())
    }

    fn crunch_desc(node: &ZilNode) -> Result<String, String> {
        if node.children.len() != 2 {
            return Err(format!(
                "Desc node doesn't have 2 children\n{}",
                format_file_location(&node)
            ));
        }

        let val = match node.children[1].node_type {
            ZilNodeType::Token(TokenType::Word) => get_token_as_word(&node.children[1]),
            ZilNodeType::Token(TokenType::Text) => get_token_as_text(&node.children[1]),
            _ => {
                return Err(format!(
                    "Desc node has invalid second child\n{}",
                    format_file_location(&node.children[1])
                ));
            }
        };

        Ok(val.unwrap())
    }

    fn crunch_vars(node: &ZilNode) -> Result<HashMap<String, VarVal>, String> {
        if node.children.len() < 3 {
            return Err(format!(
                "Vars node doesn't have enough children\n{}",
                format_file_location(&node)
            ));
        } else if node.children.len() % 2 == 0 {
            return Err(format!(
                "Vars node doesn't have an odd number of children\n{}",
                format_file_location(&node)
            ));
        }

        let mut out: HashMap<String, VarVal> = HashMap::new();

        for i in 0..(node.children.len() - 1) / 2 {
            let name = get_token_as_word(&node.children[i * 2 + 1]);
            if name.is_none() {
                return Err(format!(
                    "Vars node has non-word name child\n{}",
                    format_file_location(&node.children[i * 2 + 1])
                ));
            }

            let name = name.unwrap();
            if out.contains_key(&name) {
                return Err(format!(
                    "Vars node has duplicate variable name:{}\n{}",
                    name,
                    format_file_location(&node.children[i * 2 + 1])
                ));
            }

            let val = VarVal::parse(&node.children[i * 2 + 2]);
            if val.is_err() {
                return Err(format!(
                    "Vars node has invalid value child\n{}",
                    format_file_location(&node.children[i * 2 + 2])
                ));
            }

            out.insert(name, val.unwrap());
        }

        Ok(out)
    }
}

impl ObjectStats {
    pub fn new() -> ObjectStats {
        ObjectStats {
            basis: Vec::new(),
            all_objects: HashMap::new(),
        }
    }

    pub fn as_codex(&self) -> ObjectCodex {
        ObjectCodex {
            index: 0,
            basis: &self.basis,
            all_objects: &self.all_objects,
        }
    }
}

impl Populator for ObjectStats {
    fn add_node(&mut self, node: ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> Result<(), String> {
        for (i, node) in self.basis.iter().enumerate() {
            let mut info = ObjectInfo::new();

            if node.children.len() < 2 {
                return Err(format!(
                    "Possible object node doesn't have enough children\n{}",
                    format_file_location(&node)
                ));
            }

            let first_word = get_token_as_word(&node.children[0]).unwrap_or_default();
            if first_word != "OBJECT" {
                unreachable!();
            }

            let second_word = get_token_as_word(&node.children[1]);
            if second_word.is_none() {
                return Err(format!(
                    "Object node has non-word second child\n{}",
                    format_file_location(&node)
                ));
            }

            for c in node.children.iter().skip(2) {
                if c.node_type != ZilNodeType::Group {
                    return Err(format!(
                        "Object node has non-group child in body\n{}",
                        format_file_location(&c)
                    ));
                }

                if c.children.len() < 1 {
                    return Err(format!(
                        "Object node has unnamed group\n{}",
                        format_file_location(&c)
                    ));
                }

                let child_word = get_token_as_word(&c.children[0]);
                if child_word.is_none() {
                    return Err(format!(
                        "Object node has group with non-word first child\n{}",
                        format_file_location(&c)
                    ));
                }

                let child_word = child_word.unwrap();
                match child_word.as_str() {
                    "AKA" => match ObjectInfo::crunch_synonyms(&c) {
                        Ok(v) => info.synonyms = v,
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    "IN" => match ObjectInfo::crunch_location(&c) {
                        Ok(v) => info.loc = Some(v),
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    "DESC" => match ObjectInfo::crunch_desc(&c) {
                        Ok(v) => info.desc = Some(v),
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    "VARS" => match ObjectInfo::crunch_vars(&c) {
                        Ok(v) => info.vars = v,
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    _ => {
                        return Err(format!(
                            "Object node has unknown group:{}\n{}",
                            child_word.as_str(),
                            format_file_location(&c)
                        ));
                    }
                }
            }

            let second_word = second_word.unwrap();

            info.index = i;
            info.name = second_word.clone();
            self.all_objects.insert(second_word, info);
        }

        Ok(())
    }

    fn validate(&self, cross_ref: &crate::stats::cross_ref::CrossRef) -> Result<(), String> {
        // TODO
        // location is a room or object
        // ???

        Ok(())
    }
}

pub struct ObjectCodex<'a> {
    index: usize,
    basis: &'a Vec<ZilNode>,
    all_objects: &'a HashMap<String, ObjectInfo>,
}
pub struct ObjectCodexValue<'a> {
    pub name: &'a String,
    pub synonyms: &'a HashSet<String>,
    pub loc: &'a Option<String>,
    pub desc: &'a Option<String>,
    pub vars: &'a HashMap<String, VarVal>,
}

impl<'a> Iterator for ObjectCodex<'a> {
    type Item = ObjectCodexValue<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.basis.len() {
            None
        } else {
            self.index += 1;
            let node = &self.basis[self.index - 1];
            let key = get_token_as_word(&node.children[1]).unwrap();
            let info = self.all_objects.get(&key).unwrap();

            Some(ObjectCodexValue {
                name: &info.name,
                synonyms: &info.synonyms,
                loc: &info.loc,
                desc: &info.desc,
                vars: &info.vars,
            })
        }
    }
}

impl<'a> Codex<ObjectCodexValue<'a>> for ObjectCodex<'a> {
    fn lookup(&self, word: &str) -> Option<ObjectCodexValue<'a>> {
        let info = self.all_objects.get(word);

        if info.is_none() {
            return None;
        }

        let info = info.unwrap();

        return Some(ObjectCodexValue {
            name: &info.name,
            synonyms: &info.synonyms,
            loc: &info.loc,
            desc: &info.desc,
            vars: &info.vars,
        });
    }
}
