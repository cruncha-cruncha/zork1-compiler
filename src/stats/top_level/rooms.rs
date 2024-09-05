use std::collections::{HashMap, HashSet};

use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::{
        cross_ref::{CrossRef, Populator},
        helpers::{get_token_as_text, get_token_as_word},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use crate::stats::cross_ref::Codex;

use super::globals::{Var, VarVal};

pub struct RoomStats {
    basis: Vec<ZilNode>,
    all_rooms: HashMap<String, RoomInfo>,
}

pub struct RoomInfo {
    index: usize,
    name: String,
    desc: Option<String>,
    vars: HashMap<String, VarVal>,
    action: Option<String>, // runs all the time you're in the room
    directions: HashMap<String, Direction>,
}

impl RoomInfo {
    pub fn new() -> RoomInfo {
        RoomInfo {
            index: 0,
            name: String::new(),
            desc: None,
            vars: HashMap::new(),
            action: None,
            directions: HashMap::new(),
        }
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

    fn crunch_action(node: &ZilNode) -> Result<String, String> {
        if node.children.len() != 2 {
            return Err(format!(
                "Action node doesn't have 2 children\n{}",
                format_file_location(&node)
            ));
        }

        let word = get_token_as_word(&node.children[1]);
        if word.is_none() {
            return Err(format!(
                "Action node has non-word second child\n{}",
                format_file_location(&node.children[1])
            ));
        }

        Ok(word.unwrap())
    }

    fn crunch_direction(node: &ZilNode) -> Result<Direction, String> {
        if node.children.len() < 2 {
            return Err(format!(
                "Direction node doesn't have enough children\n{}",
                format_file_location(&node)
            ));
        } else if node.children.len() > 3 {
            return Err(format!(
                "Direction node has too many children\n{}",
                format_file_location(&node)
            ));
        }

        let first_word = get_token_as_word(&node.children[0]).unwrap_or_default();

        if node.children.len() == 2 {
            let text = get_token_as_text(&node.children[1]);
            if text.is_none() {
                return Err(format!(
                    "Text-type direction node has non-text second child\n{}",
                    format_file_location(&node.children[1])
                ));
            }

            return Ok(Direction {
                name: first_word,
                kind: DirectionType::TEXT,
                thing: text.unwrap(),
            });
        }

        let second_word = get_token_as_word(&node.children[1]);
        if second_word.is_none() {
            return Err(format!(
                "Room or routine type direction node has non-word second child\n{}",
                format_file_location(&node.children[1])
            ));
        }

        let second_word = second_word.unwrap();
        if second_word == "PER" {
            let routine = get_token_as_word(&node.children[2]);
            if routine.is_none() {
                return Err(format!(
                    "Routine-type direction node has non-word third child\n{}",
                    format_file_location(&node.children[2])
                ));
            }

            return Ok(Direction {
                name: first_word,
                kind: DirectionType::ROUTINE,
                thing: routine.unwrap(),
            });
        } else if second_word == "TO" {
            let room = get_token_as_word(&node.children[2]);
            if room.is_none() {
                return Err(format!(
                    "Room-type direction node has non-word third child\n{}",
                    format_file_location(&node.children[2])
                ));
            }

            return Ok(Direction {
                name: first_word,
                kind: DirectionType::ROOM,
                thing: room.unwrap(),
            });
        }

        return Err(format!(
            "Direction node has invalid second word:{}\n{}",
            second_word,
            format_file_location(&node.children[1])
        ));
    }
}

pub struct Direction {
    pub name: String,
    pub kind: DirectionType,
    pub thing: String, // TEXT, name of ROUTINE, or name of ROOM
}

#[derive(Clone, Copy)]
pub enum DirectionType {
    TEXT,    // SW <TEXT>
    ROUTINE, // SW PER <ROUTINE>
    ROOM,    // SW TO <ROOM>
}

impl RoomStats {
    pub fn new() -> RoomStats {
        RoomStats {
            basis: Vec::new(),
            all_rooms: HashMap::new(),
        }
    }

    pub fn as_codex(&self) -> RoomCodex {
        RoomCodex {
            index: 0,
            basis: &self.basis,
            all_rooms: &self.all_rooms,
        }
    }
}

impl Populator for RoomStats {
    fn add_node(&mut self, node: ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> Result<(), String> {
        for (i, node) in self.basis.iter().enumerate() {
            let mut info = RoomInfo::new();

            if node.children.len() < 2 {
                return Err(format!(
                    "Possible room node doesn't have enough children\n{}",
                    format_file_location(&node)
                ));
            }

            let first_word = get_token_as_word(&node.children[0]).unwrap_or_default();
            if first_word != "ROOM" {
                unreachable!();
            }

            let second_word = get_token_as_word(&node.children[1]);
            if second_word.is_none() {
                return Err(format!(
                    "Room node has non-word second child\n{}",
                    format_file_location(&node)
                ));
            }

            for c in node.children.iter().skip(2) {
                if c.node_type != ZilNodeType::Group {
                    return Err(format!(
                        "Room node has non-group child in body\n{}",
                        format_file_location(&c)
                    ));
                }

                if c.children.len() < 1 {
                    return Err(format!(
                        "Room node has unnamed group\n{}",
                        format_file_location(&c)
                    ));
                }

                let child_word = get_token_as_word(&c.children[0]);
                if child_word.is_none() {
                    return Err(format!(
                        "Room node has group with non-word first child\n{}",
                        format_file_location(&c)
                    ));
                }

                let child_word = child_word.unwrap();
                match child_word.as_str() {
                    "DESC" => match RoomInfo::crunch_desc(&c) {
                        Ok(v) => info.desc = Some(v),
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    "VARS" => match RoomInfo::crunch_vars(&c) {
                        Ok(v) => info.vars = v,
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    "ACTION" => match RoomInfo::crunch_action(&c) {
                        Ok(v) => info.action = Some(v),
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    _ => match RoomInfo::crunch_direction(&c) {
                        Ok(val) => match info.directions.insert(val.name.clone(), val) {
                            Some(old_val) => {
                                return Err(format!(
                                    "Room node has duplicate direction:{}\n{}",
                                    old_val.name,
                                    format_file_location(&c)
                                ));
                            }
                            None => (),
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    },
                }
            }

            let second_word = second_word.unwrap();

            info.index = i;
            info.name = second_word.clone();
            self.all_rooms.insert(second_word, info);
        }

        Ok(())
    }

    fn validate(&self, cross_ref: &CrossRef) -> Result<(), String> {
        // TODO
        // check that action routines exists
        // check that direction routines exist
        // check that direction rooms exist
        // check that variable values exist??

        Ok(())
    }
}

pub struct RoomCodex<'a> {
    index: usize,
    basis: &'a Vec<ZilNode>,
    all_rooms: &'a HashMap<String, RoomInfo>,
}
pub struct RoomCodexValue<'a> {
    pub name: &'a String,
    pub desc: &'a Option<String>,
    pub vars: &'a HashMap<String, VarVal>,
    pub action: &'a Option<String>,
    pub directions: &'a HashMap<String, Direction>,
}

impl<'a> Iterator for RoomCodex<'a> {
    type Item = RoomCodexValue<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.basis.len() {
            None
        } else {
            self.index += 1;
            let node = &self.basis[self.index - 1];
            let key = get_token_as_word(&node.children[1]).unwrap();
            let info = self.all_rooms.get(&key).unwrap();

            Some(RoomCodexValue {
                name: &info.name,
                desc: &info.desc,
                vars: &info.vars,
                action: &info.action,
                directions: &info.directions,
            })
        }
    }
}

impl<'a> Codex<RoomCodexValue<'a>> for RoomCodex<'a> {
    fn lookup(&self, word: &str) -> Option<RoomCodexValue<'a>> {
        let info = self.all_rooms.get(word);

        if info.is_none() {
            return None;
        }

        let info = info.unwrap();

        return Some(RoomCodexValue {
            name: &info.name,
            desc: &info.desc,
            vars: &info.vars,
            action: &info.action,
            directions: &info.directions,
        });
    }
}
