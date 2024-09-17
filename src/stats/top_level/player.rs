use std::collections::BTreeMap;

use crate::stats::helpers::{
    get_token_as_word, num_children, num_children_more_than, DescType, Helpers, ValidationResult,
};
use crate::zil::node::ZilNodeType;
use crate::zil::{file_table::format_file_location, node::ZilNode};

use crate::stats::cross_ref::{Codex, CrossRef, Populator};

use super::objects::{ObjectCodex, ObjectLocation};

pub struct PlayerStats {
    basis: Vec<ZilNode>,
    info: PlayerInfo,
}

pub struct PlayerInfo {
    pub room: Option<String>,
    pub desc: Option<DescType>,
    pub vars: BTreeMap<String, i32>,
    pub actions: PlayerActions,
    pub objects: BTreeMap<String, Vec<String>>,
}

pub struct PlayerActions {
    pub enter: Option<String>,  // when player enters any room
    pub exit: Option<String>,   // when player exits any room (pass as currentRoom)
    pub always: Option<String>, // after every command
}

impl PlayerActions {
    pub fn new() -> PlayerActions {
        PlayerActions {
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
        for info in object_codex {
            for copy in info.copies.iter() {
                match copy.loc {
                    ObjectLocation::Player => match self.info.objects.get_mut(&copy.name) {
                        Some(list) => {
                            list.push(copy.id.clone());
                        }
                        None => {
                            self.info
                                .objects
                                .insert(copy.name.clone(), vec![copy.id.clone()]);
                        }
                    },
                    _ => (),
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
            vars: BTreeMap::new(),
            actions: PlayerActions::new(),
            objects: BTreeMap::new(),
        }
    }

    fn crunch_room(node: &ZilNode) -> ValidationResult<String> {
        if let Err(e) = num_children(node, 2) {
            return Err(vec![e]);
        }

        let word = match get_token_as_word(&node.children[1]) {
            Ok(v) => v,
            Err(e) => {
                return Err(vec![e]);
            }
        };

        Ok(word)
    }
}

impl Populator for PlayerStats {
    fn add_node(&mut self, node: ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> ValidationResult<()> {
        if self.basis.len() == 0 {
            println!("WARNING: PlayerStats has no basis. Please provide a <PLAYER ... > cluster.");
            println!("Player will spawn in a random room.");
            return Ok(());
        }

        if self.basis.len() > 1 {
            return Err(vec!["Too many PLAYER definitions".to_string()]);
        }

        let node = &self.basis[0];
        if let Err(e) = num_children_more_than(node, 1) {
            return Err(vec![e]);
        }

        let mut info = PlayerInfo::new();
        for c in node.children.iter().skip(1) {
            if c.node_type != ZilNodeType::Group {
                return Err(vec![format!(
                    "Expected group, found \n{}",
                    format_file_location(&c)
                )]);
            }

            if let Err(e) = num_children_more_than(c, 1) {
                return Err(vec![e]);
            }

            let child_word = match get_token_as_word(&c.children[0]) {
                Ok(v) => v,
                Err(e) => {
                    return Err(vec![e]);
                }
            };

            match child_word.as_str() {
                "DESC" => match Helpers::crunch_desc(&c) {
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
                "VARS" => match Helpers::crunch_vars(&c) {
                    Ok(v) => info.vars = v,
                    Err(e) => {
                        return Err(e);
                    }
                },
                "ACT-ENTER" => {
                    if info.actions.enter.is_some() {
                        return Err(vec![format!(
                            "Duplicate enter action node\n{}",
                            format_file_location(&c)
                        )]);
                    } else {
                        match Helpers::crunch_action(&c) {
                            Ok(v) => info.actions.enter = Some(v),
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                }
                "ACT-EXIT" => {
                    if info.actions.exit.is_some() {
                        return Err(vec![format!(
                            "Duplicate exit action node\n{}",
                            format_file_location(&c)
                        )]);
                    } else {
                        match Helpers::crunch_action(&c) {
                            Ok(v) => info.actions.exit = Some(v),
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                }
                "ACT-ALWAYS" => {
                    if info.actions.always.is_some() {
                        return Err(vec![format!(
                            "Duplicate always action node\n{}",
                            format_file_location(&c)
                        )]);
                    } else {
                        match Helpers::crunch_action(&c) {
                            Ok(v) => info.actions.always = Some(v),
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                }
                _ => {
                    return Err(vec![format!(
                        "Player node has unknown group:{}\n{}",
                        child_word.as_str(),
                        format_file_location(&c)
                    )]);
                }
            }
        }

        self.info = info;

        Ok(())
    }

    fn validate(&self, cross_ref: &CrossRef) -> ValidationResult<()> {
        let mut errors: Vec<String> = Vec::new();
        let routine_codex = cross_ref.routines.as_codex();
        let room_codex = cross_ref.rooms.as_codex();

        if self.info.room.is_some() {
            let room = self.info.room.as_ref().unwrap();
            if room_codex.lookup(room).is_none() {
                errors.push(format!(
                    "Player room {} does not exist",
                    self.info.room.as_ref().unwrap()
                ));
            }
        }

        if self.info.actions.enter.is_some() {
            let action = self.info.actions.enter.as_ref().unwrap();
            if routine_codex.lookup(action).is_none() {
                errors.push(format!(
                    "Player has invalid enter action routine: {}",
                    action
                ));
            }
        }

        if self.info.actions.exit.is_some() {
            let action = self.info.actions.exit.as_ref().unwrap();
            if routine_codex.lookup(action).is_none() {
                errors.push(format!(
                    "Player has invalid exit action routine: {}",
                    action
                ));
            }
        }

        if self.info.actions.always.is_some() {
            let action = self.info.actions.always.as_ref().unwrap();
            if routine_codex.lookup(action).is_none() {
                errors.push(format!(
                    "Player has invalid always action routine: {}",
                    action
                ));
            }
        }

        Ok(())
    }
}
