use std::collections::BTreeMap;

use crate::{
    stats::{
        cross_ref::{CrossRef, Populator},
        helpers::{
            get_token_as_text, get_token_as_word, num_children_between, num_children_more_than,
            DescType, Helpers, ValidationResult,
        },
    },
    zil::{
        file_table::format_file_location,
        node::{ZilNode, ZilNodeType},
    },
};

use crate::stats::cross_ref::Codex;

use super::objects::{ObjectCodex, ObjectLocation};

pub struct RoomStats {
    basis: Vec<ZilNode>,
    all_rooms: BTreeMap<String, RoomInfo>,
}

pub struct RoomInfo {
    index: usize,
    name: String,
    desc: Option<DescType>,
    vars: BTreeMap<String, i32>,
    actions: RoomActions,
    directions: BTreeMap<String, Direction>,
    objects: BTreeMap<String, Vec<String>>,
}

pub struct RoomActions {
    pub enter: Option<String>,  // when player enters this room
    pub exit: Option<String>,   // when player exits this room (pass as currentRoom)
    pub always: Option<String>, // after every command while in this room
}

impl RoomActions {
    pub fn new() -> RoomActions {
        RoomActions {
            enter: None,
            exit: None,
            always: None,
        }
    }
}

impl RoomInfo {
    pub fn new() -> RoomInfo {
        RoomInfo {
            index: 0,
            name: String::new(),
            desc: None,
            vars: BTreeMap::new(),
            actions: RoomActions::new(),
            directions: BTreeMap::new(),
            objects: BTreeMap::new(),
        }
    }

    fn crunch_direction(node: &ZilNode) -> Result<Direction, String> {
        num_children_between(node, 2, 3)?;

        let first_word = get_token_as_word(&node.children[0]).unwrap_or_default();

        if node.children.len() == 2 {
            let text = get_token_as_text(&node.children[1])?;

            return Ok(Direction {
                name: first_word,
                kind: DirectionType::TEXT,
                thing: text,
            });
        }

        let second_word = get_token_as_word(&node.children[1])?;

        if second_word == "PER" {
            let routine = get_token_as_word(&node.children[2])?;

            return Ok(Direction {
                name: first_word,
                kind: DirectionType::ROUTINE,
                thing: routine,
            });
        } else if second_word == "TO" {
            let room = get_token_as_word(&node.children[2])?;

            return Ok(Direction {
                name: first_word,
                kind: DirectionType::ROOM,
                thing: room,
            });
        }

        return Err(format!(
            "Possible direction node has invalid second word:{}\n{}",
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
            all_rooms: BTreeMap::new(),
        }
    }

    pub fn as_codex(&self) -> RoomCodex {
        RoomCodex {
            index: 0,
            basis: &self.basis,
            all_rooms: &self.all_rooms,
        }
    }

    pub fn nest_objects(&mut self, object_codex: ObjectCodex) {
        for info in object_codex {
            for copy in info.copies.iter() {
                match copy.loc {
                    ObjectLocation::Room(ref name) => {
                        let room = match self.all_rooms.get_mut(name) {
                            Some(room) => room,
                            None => panic!(),
                        };

                        match room.objects.get_mut(&copy.name) {
                            Some(list) => {
                                list.push(copy.id.clone());
                            }
                            _ => {
                                room.objects
                                    .insert(copy.name.clone(), vec![copy.id.clone()]);
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}

impl Populator for RoomStats {
    fn add_node(&mut self, node: ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> ValidationResult<()> {
        let mut errors: Vec<String> = Vec::new();
        for (i, node) in self.basis.iter().enumerate() {
            let mut room_errors: Vec<String> = Vec::new();
            let mut info = RoomInfo::new();

            match num_children_more_than(node, 1) {
                Ok(_) => (),
                Err(e) => {
                    room_errors.push(e);
                    continue;
                }
            }

            let room_name = match get_token_as_word(&node.children[1]) {
                Ok(v) => Some(v),
                Err(e) => {
                    room_errors.push(e);
                    None
                }
            };

            for c in node.children.iter().skip(2) {
                if c.node_type != ZilNodeType::Group {
                    room_errors.push(format!(
                        "Expected group, found \n{}",
                        format_file_location(&c)
                    ));
                    continue;
                }

                if let Err(e) = num_children_more_than(c, 0) {
                    room_errors.push(e);
                    continue;
                }

                let group_name = match get_token_as_word(&c.children[0]) {
                    Ok(v) => v,
                    Err(e) => {
                        room_errors.push(e);
                        continue;
                    }
                };

                match group_name.as_str() {
                    "DESC" => {
                        if info.desc.is_some() {
                            room_errors
                                .push(format!("Duplicate desc node\n{}", format_file_location(&c)));
                        } else {
                            match Helpers::crunch_desc(&c) {
                                Ok(v) => info.desc = Some(v),
                                Err(mut e) => {
                                    room_errors.append(&mut e);
                                }
                            }
                        }
                    }
                    "VARS" => {
                        if info.vars.len() > 0 {
                            room_errors
                                .push(format!("Duplicate vars node\n{}", format_file_location(&c)));
                        } else {
                            match Helpers::crunch_vars(&c) {
                                Ok(v) => info.vars = v,
                                Err(mut e) => {
                                    room_errors.append(&mut e);
                                }
                            }
                        }
                    }
                    "ACT-ENTER" => {
                        if info.actions.enter.is_some() {
                            room_errors.push(format!(
                                "Duplicate enter action node\n{}",
                                format_file_location(&c)
                            ));
                        } else {
                            match Helpers::crunch_action(&c) {
                                Ok(v) => info.actions.enter = Some(v),
                                Err(mut e) => {
                                    room_errors.append(&mut e);
                                }
                            }
                        }
                    }
                    "ACT-EXIT" => {
                        if info.actions.exit.is_some() {
                            room_errors.push(format!(
                                "Duplicate exit action node\n{}",
                                format_file_location(&c)
                            ));
                        } else {
                            match Helpers::crunch_action(&c) {
                                Ok(v) => info.actions.exit = Some(v),
                                Err(mut e) => {
                                    room_errors.append(&mut e);
                                }
                            }
                        }
                    }
                    "ACT-ALWAYS" => {
                        if info.actions.always.is_some() {
                            room_errors.push(format!(
                                "Duplicate always action node\n{}",
                                format_file_location(&c)
                            ));
                        } else {
                            match Helpers::crunch_action(&c) {
                                Ok(v) => info.actions.always = Some(v),
                                Err(mut e) => {
                                    room_errors.append(&mut e);
                                }
                            }
                        }
                    }
                    _ => match RoomInfo::crunch_direction(&c) {
                        Ok(val) => match info.directions.insert(val.name.clone(), val) {
                            Some(old_val) => {
                                room_errors.push(format!(
                                    "Room node has duplicate group node:{}\n{}",
                                    old_val.name,
                                    format_file_location(&c)
                                ));
                            }
                            None => (),
                        },
                        Err(e) => {
                            room_errors.push(e);
                        }
                    },
                }
            }

            if room_errors.len() > 0 {
                errors.append(&mut room_errors);
                continue;
            }

            let room_name = room_name.unwrap();

            info.index = i;
            info.name = room_name.clone();
            match self.all_rooms.insert(room_name, info) {
                Some(old_val) => {
                    errors.push(format!(
                        "Duplicate room name:{}\n{}",
                        old_val.name,
                        format_file_location(&node)
                    ));
                }
                None => (),
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(())
    }

    fn validate(&self, cross_ref: &CrossRef) -> ValidationResult<()> {
        let mut errors: Vec<String> = Vec::new();

        for key in self.all_rooms.keys() {
            if CrossRef::name_is_illegal(key) {
                errors.push(format!("Illegal room name: {}", key));
            }
        }

        let routine_codex = cross_ref.routines.as_codex();
        let room_codex = self.as_codex();

        for (key, info) in self.all_rooms.iter() {
            if info.actions.enter.is_some() {
                let action = info.actions.enter.as_ref().unwrap();
                if routine_codex.lookup(action).is_none() {
                    errors.push(format!(
                        "Room {} has invalid enter action routine: {}",
                        key, action
                    ));
                }
            }

            if info.actions.exit.is_some() {
                let action = info.actions.exit.as_ref().unwrap();
                if routine_codex.lookup(action).is_none() {
                    errors.push(format!(
                        "Room {} has invalid exit action routine: {}",
                        key, action
                    ));
                }
            }

            if info.actions.always.is_some() {
                let action = info.actions.always.as_ref().unwrap();
                if routine_codex.lookup(action).is_none() {
                    errors.push(format!(
                        "Room {} has invalid always action routine: {}",
                        key, action
                    ));
                }
            }

            for direction in info.directions.values() {
                match direction.kind {
                    DirectionType::TEXT => (),
                    DirectionType::ROUTINE => {
                        if routine_codex.lookup(&direction.thing).is_none() {
                            errors.push(format!(
                                "Room {} has invalid routine direction: {}",
                                key, direction.thing
                            ));
                        }
                    }
                    DirectionType::ROOM => {
                        if room_codex.lookup(&direction.thing).is_none() {
                            errors.push(format!(
                                "Room {} has invalid room direction: {}",
                                key, direction.thing
                            ));
                        }
                    }
                }
            }

            if info.desc.is_some() {
                match info.desc.as_ref().unwrap() {
                    DescType::Routine(routine) => {
                        if routine_codex.lookup(routine).is_none() {
                            errors.push(format!(
                                "Room {} has invalid desc routine: {}",
                                key, routine
                            ));
                        }
                    }
                    DescType::Text(..) => (),
                }
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(())
    }
}

pub struct RoomCodex<'a> {
    index: usize,
    basis: &'a Vec<ZilNode>,
    all_rooms: &'a BTreeMap<String, RoomInfo>,
}
pub struct RoomCodexValue<'a> {
    pub name: &'a String,
    pub desc: &'a Option<DescType>,
    pub vars: &'a BTreeMap<String, i32>,
    pub actions: &'a RoomActions,
    pub directions: &'a BTreeMap<String, Direction>,
    pub objects: &'a BTreeMap<String, Vec<String>>,
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
                actions: &info.actions,
                directions: &info.directions,
                objects: &info.objects,
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
            actions: &info.actions,
            directions: &info.directions,
            objects: &info.objects,
        });
    }
}
