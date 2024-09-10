use std::collections::{HashMap, HashSet};

use crate::{
    stats::{
        cross_ref::{Codex, CrossRef, Populator},
        helpers::{get_token_as_number, get_token_as_text, get_token_as_word},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

pub struct ObjectStats {
    basis: Vec<ZilNode>,
    all_objects: HashMap<String, ObjectInfo>,
}

pub struct ObjectInfo {
    index: usize,
    name: String,
    synonyms: HashSet<String>,
    loc: Option<String>, // can be room or object
    desc: Option<DescType>,
    vars: HashMap<String, i32>,
    actions: ObjectActions,
    objects: Vec<String>,
}

pub enum DescType {
    Routine(String),
    Text(String),
}

pub struct ObjectActions {
    pub in_room: Option<String>, // when this object is in the same room as player, can be nested within an object in the same room (pass as PRSO)
    pub in_player: Option<String>, // when this object is in player inventory, ditto
    pub enter_player: Option<String>, // when this object enters player inventory, ditto
    pub exit_player: Option<String>, // when this object leaves player inventory, ditto
    pub prso: Option<String>,    // when this object is used as the PRSO
    pub prsi: Option<String>,    // when this object is used as the PRSI
}

impl ObjectActions {
    pub fn new() -> ObjectActions {
        ObjectActions {
            in_room: None,
            in_player: None,
            enter_player: None,
            exit_player: None,
            prso: None,
            prsi: None,
        }
    }
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
            actions: ObjectActions::new(),
            objects: Vec::new(),
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

    pub fn nest_objects(&mut self) {
        let mut objects_in_objects: HashMap<String, Vec<String>> = HashMap::new();
        for (key, info) in self.all_objects.iter() {
            if info.loc.is_some() {
                let loc = info.loc.as_ref().unwrap();

                if let Some(v) = objects_in_objects.get_mut(loc) {
                    v.push(key.clone());
                } else {
                    objects_in_objects.insert(loc.clone(), vec![key.clone()]);
                }
            }
        }

        for (key, info) in self.all_objects.iter_mut() {
            if let Some(v) = objects_in_objects.get(key) {
                info.objects = v.clone();
            }
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
                    "ACT-IN-ROOM" => match ObjectInfo::crunch_action(&c) {
                        Ok(v) => info.actions.in_room = Some(v),
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    "ACT-IN-PLAYER" => match ObjectInfo::crunch_action(&c) {
                        Ok(v) => info.actions.in_player = Some(v),
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    "ACT-ADD" => match ObjectInfo::crunch_action(&c) {
                        Ok(v) => info.actions.enter_player = Some(v),
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    "ACT-REMOVE" => match ObjectInfo::crunch_action(&c) {
                        Ok(v) => info.actions.exit_player = Some(v),
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    "ACT-PRSO" => match ObjectInfo::crunch_action(&c) {
                        Ok(v) => info.actions.prso = Some(v),
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    "ACT-PRSI" => match ObjectInfo::crunch_action(&c) {
                        Ok(v) => info.actions.prsi = Some(v),
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
            match self.all_objects.insert(second_word, info) {
                Some(old_val) => {
                    return Err(format!(
                        "Duplicate object name: {}\n{}",
                        old_val.name,
                        format_file_location(&node)
                    ));
                }
                None => {}
            }
        }

        Ok(())
    }

    fn validate(&self, cross_ref: &CrossRef) -> Result<(), String> {
        for key in self.all_objects.keys() {
            if CrossRef::name_is_illegal(key) {
                return Err(format!("Illegal object name: {}", key));
            }
        }

        let routine_codex = cross_ref.routines.as_codex();
        let room_codex = cross_ref.rooms.as_codex();
        let object_codex = self.as_codex();

        for (key, info) in self.all_objects.iter() {
            if info.loc.is_some() {
                let loc = info.loc.as_ref().unwrap();

                if loc == "PLAYER" {
                    continue;
                }

                if room_codex.lookup(loc).is_none() && object_codex.lookup(loc).is_none() {
                    return Err(format!("Object {} has invalid location: {}", key, loc));
                }
            }

            if info.desc.is_some() {
                match info.desc.as_ref().unwrap() {
                    DescType::Routine(routine) => {
                        if routine_codex.lookup(routine).is_none() {
                            return Err(format!(
                                "Object {} has invalid desc routine: {}",
                                key, routine
                            ));
                        }
                    }
                    DescType::Text(_) => (),
                }
            }

            if info.actions.in_room.is_some() {
                let action = info.actions.in_room.as_ref().unwrap();
                if routine_codex.lookup(action).is_none() {
                    return Err(format!(
                        "Object {} has invalid in-room action routine: {}",
                        key, action
                    ));
                }
            }

            if info.actions.in_player.is_some() {
                let action = info.actions.in_player.as_ref().unwrap();
                if routine_codex.lookup(action).is_none() {
                    return Err(format!(
                        "Object {} has invalid in-player action routine: {}",
                        key, action
                    ));
                }
            }

            if info.actions.enter_player.is_some() {
                let action = info.actions.enter_player.as_ref().unwrap();
                if routine_codex.lookup(action).is_none() {
                    return Err(format!(
                        "Object {} has invalid add-player action routine: {}",
                        key, action
                    ));
                }
            }

            if info.actions.exit_player.is_some() {
                let action = info.actions.exit_player.as_ref().unwrap();
                if routine_codex.lookup(action).is_none() {
                    return Err(format!(
                        "Object {} has invalid remove-player action routine: {}",
                        key, action
                    ));
                }
            }

            if info.actions.prso.is_some() {
                let action = info.actions.prso.as_ref().unwrap();
                if routine_codex.lookup(action).is_none() {
                    return Err(format!(
                        "Object {} has invalid prso action routine: {}",
                        key, action
                    ));
                }
            }

            if info.actions.prsi.is_some() {
                let action = info.actions.prsi.as_ref().unwrap();
                if routine_codex.lookup(action).is_none() {
                    return Err(format!(
                        "Object {} has invalid prsi action routine: {}",
                        key, action
                    ));
                }
            }
        }

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
    pub desc: &'a Option<DescType>,
    pub vars: &'a HashMap<String, i32>,
    pub actions: &'a ObjectActions,
    pub objects: &'a Vec<String>,
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
                actions: &info.actions,
                objects: &info.objects,
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
            actions: &info.actions,
            objects: &info.objects,
        });
    }
}
