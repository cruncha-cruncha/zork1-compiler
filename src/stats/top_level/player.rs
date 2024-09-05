use std::collections::HashSet;

use crate::stats::helpers::get_token_as_word;
use crate::zil::node::ZilNodeType;
use crate::zil::{file_table::format_file_location, node::ZilNode};

use crate::stats::cross_ref::{CrossRef, Populator};

pub struct PlayerStats {
    basis: Vec<ZilNode>,
    info: Option<PlayerInfo>,
}

pub struct PlayerInfo {
    room: Option<String>,
}

impl PlayerStats {
    pub fn new() -> PlayerStats {
        PlayerStats {
            basis: Vec::new(),
            info: None,
        }
    }

    pub fn get_room(&self) -> Option<String> {
        if self.info.is_none() {
            return None;
        }

        self.info.as_ref().unwrap().room.clone()
    }
}

impl PlayerInfo {
    pub fn new() -> PlayerInfo {
        PlayerInfo { room: None }
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
                "ROOM" => match PlayerInfo::crunch_room(&c) {
                    Ok(v) => info.room = Some(v),
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

        self.info = Some(info);

        Ok(())
    }

    fn validate(&self, _cross_ref: &CrossRef) -> Result<(), String> {
        // TODO

        Ok(())
    }
}
