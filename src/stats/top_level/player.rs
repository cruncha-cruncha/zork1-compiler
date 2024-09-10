use std::collections::HashMap;

use crate::stats::helpers::{get_token_as_number, get_token_as_text, get_token_as_word};
use crate::zil::node::{TokenType, ZilNodeType};
use crate::zil::{file_table::format_file_location, node::ZilNode};

use crate::stats::cross_ref::{Codex, CrossRef, Populator};

use super::objects::{DescType, ObjectCodex};

pub struct PlayerStats {
    basis: Vec<ZilNode>,
    info: PlayerInfo,
}

pub struct PlayerInfo {
    pub room: Option<String>,
    pub desc: Option<DescType>,
    pub vars: HashMap<String, i32>,
    pub actions: PlayerActions,
    pub objects: Vec<String>,
}

pub struct PlayerActions {
    pub first_enter: Option<String>, // when player enters a room for the first time
    pub enter: Option<String>,       // when player enters any room
    pub exit: Option<String>,        // when player exits any room (pass as currentRoom)
    pub always: Option<String>,      // after every command
}

impl PlayerActions {
    pub fn new() -> PlayerActions {
        PlayerActions {
            first_enter: None,
            enter: None,
            exit: None,
            always: None,
        }
    }
}

impl PlayerStats {
    pub fn new() -> PlayerStats {
        PlayerStats {
            basis: Vec::new(),
            info: PlayerInfo::new(),
        }
    }

    pub fn get_info(&self) -> &PlayerInfo {
        &self.info
    }

    pub fn nest_objects(&mut self, object_codex: ObjectCodex) {
        for object in object_codex {
            if object.loc.is_some() {
                let key = object.loc.as_ref().unwrap();
                if key == "PLAYER" {
                    self.info.objects.push(object.name.clone());
                }
            }
        }
    }
}

impl PlayerInfo {
    pub fn new() -> PlayerInfo {
        PlayerInfo {
            room: None,
            desc: None,
            vars: HashMap::new(),
            actions: PlayerActions::new(),
            objects: Vec::new(),
        }
    }

    fn crunch_desc(node: &ZilNode) -> Result<DescType, String> {
        if node.children.len() != 2 {
            return Err(format!(
                "Desc node doesn't have 2 children\n{}",
                format_file_location(&node)
            ));
        }

        match node.children[1].node_type {
            ZilNodeType::Token(TokenType::Word) => Ok(DescType::Routine(
                get_token_as_word(&node.children[1]).unwrap(),
            )),
            ZilNodeType::Token(TokenType::Text) => Ok(DescType::Text(
                get_token_as_text(&node.children[1]).unwrap(),
            )),
            _ => {
                return Err(format!(
                    "Desc node has invalid second child\n{}",
                    format_file_location(&node.children[1])
                ));
            }
        }
    }

    fn crunch_room(node: &ZilNode) -> Result<String, String> {
        if node.children.len() != 2 {
            return Err(format!(
                "Room group does not have two children\n{}",
                format_file_location(&node)
            ));
        }

        let word = get_token_as_word(&node.children[1]);
        if word.is_none() {
            return Err(format!(
                "Room group does not have a word as second child\n{}",
                format_file_location(&node)
            ));
        }

        Ok(word.unwrap())
    }

    fn crunch_vars(node: &ZilNode) -> Result<HashMap<String, i32>, String> {
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

        let mut out: HashMap<String, i32> = HashMap::new();

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

            let val = get_token_as_number(&node.children[i * 2 + 2]);
            if val.is_none() {
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
}

impl Populator for PlayerStats {
    fn add_node(&mut self, node: ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> Result<(), String> {
        if self.basis.len() == 0 {
            println!("WARNING: PlayerStats has no basis. Please provide a <PLAYER ... > cluster.");
            println!("Player will spawn in a random room.");
            return Ok(());
        }

        if self.basis.len() > 1 {
            return Err("Too many possible PLAYER definitions".to_string());
        }

        let node = &self.basis[0];
        if node.children.len() < 2 {
            return Err(format!(
                "Possible player node doesn't have enough children\n{}",
                format_file_location(&node)
            ));
        }

        let first_word: String = get_token_as_word(&node.children[0]).unwrap_or_default();
        if first_word != "PLAYER" {
            unreachable!();
        }

        let mut info = PlayerInfo::new();
        for c in node.children.iter().skip(1) {
            if c.node_type != ZilNodeType::Group {
                return Err(format!(
                    "Player node has non-group child in body\n{}",
                    format_file_location(&c)
                ));
            }

            if c.children.len() < 1 {
                return Err(format!(
                    "Player node has unnamed group\n{}",
                    format_file_location(&c)
                ));
            }

            let child_word = get_token_as_word(&c.children[0]);
            if child_word.is_none() {
                return Err(format!(
                    "Player node has group with non-word first child\n{}",
                    format_file_location(&c)
                ));
            }

            let child_word = child_word.unwrap();
            match child_word.as_str() {
                "DESC" => match PlayerInfo::crunch_desc(&c) {
                    Ok(v) => info.desc = Some(v),
                    Err(e) => {
                        return Err(e);
                    }
                },
                "ROOM" => match PlayerInfo::crunch_room(&c) {
                    Ok(v) => info.room = Some(v),
                    Err(e) => {
                        return Err(e);
                    }
                },
                "VARS" => match PlayerInfo::crunch_vars(&c) {
                    Ok(v) => info.vars = v,
                    Err(e) => {
                        return Err(e);
                    }
                },
                "ACT-FIRST" => match PlayerInfo::crunch_action(&c) {
                    Ok(v) => info.actions.first_enter = Some(v),
                    Err(e) => {
                        return Err(e);
                    }
                },
                "ACT-ENTER" => match PlayerInfo::crunch_action(&c) {
                    Ok(v) => info.actions.enter = Some(v),
                    Err(e) => {
                        return Err(e);
                    }
                },
                "ACT-EXIT" => match PlayerInfo::crunch_action(&c) {
                    Ok(v) => info.actions.exit = Some(v),
                    Err(e) => {
                        return Err(e);
                    }
                },
                "ACT-ALWAYS" => match PlayerInfo::crunch_action(&c) {
                    Ok(v) => info.actions.always = Some(v),
                    Err(e) => {
                        return Err(e);
                    }
                },
                _ => {
                    return Err(format!(
                        "Player node has unknown group:{}\n{}",
                        child_word.as_str(),
                        format_file_location(&c)
                    ));
                }
            }
        }

        self.info = info;

        Ok(())
    }

    fn validate(&self, cross_ref: &CrossRef) -> Result<(), String> {
        let routine_codex = cross_ref.routines.as_codex();
        let room_codex = cross_ref.rooms.as_codex();

        if self.info.room.is_some() {
            let room = self.info.room.as_ref().unwrap();
            if room_codex.lookup(room).is_none() {
                return Err(format!(
                    "Player room {} does not exist",
                    self.info.room.as_ref().unwrap()
                ));
            }
        }

        if self.info.actions.first_enter.is_some() {
            let action = self.info.actions.first_enter.as_ref().unwrap();
            if routine_codex.lookup(action).is_none() {
                return Err(format!(
                    "Player has invalid first-enter action routine: {}",
                    action
                ));
            }
        }

        if self.info.actions.enter.is_some() {
            let action = self.info.actions.enter.as_ref().unwrap();
            if routine_codex.lookup(action).is_none() {
                return Err(format!(
                    "Player has invalid enter action routine: {}",
                    action
                ));
            }
        }

        if self.info.actions.exit.is_some() {
            let action = self.info.actions.exit.as_ref().unwrap();
            if routine_codex.lookup(action).is_none() {
                return Err(format!(
                    "Player has invalid exit action routine: {}",
                    action
                ));
            }
        }

        if self.info.actions.always.is_some() {
            let action = self.info.actions.always.as_ref().unwrap();
            if routine_codex.lookup(action).is_none() {
                return Err(format!(
                    "Player has invalid always action routine: {}",
                    action
                ));
            }
        }

        Ok(())
    }
}
