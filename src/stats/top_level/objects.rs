use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::{
    stats::{
        cross_ref::{Codex, CrossRef, Populator},
        helpers::{
            get_token_as_number, get_token_as_word, i32_to_usize, num_children_between,
            num_children_more_than, DescType, Helpers, ValidationResult,
        },
    },
    zil::{
        file_table::format_file_location,
        node::{ZilNode, ZilNodeType},
    },
};

pub struct ObjectStats {
    basis: Vec<ZilNode>,
    all_objects: BTreeMap<String, ObjectInfo>,
}

pub struct ObjectInfo {
    index: usize,
    name: String,
    synonyms: BTreeSet<String>,
    desc: Option<DescType>,
    vars: BTreeMap<String, i32>,
    copies: Vec<ObjectInstance>,
    actions: ObjectActions,
}

/*
<OBJECT LOG
    (COPY
        <PLAYER (VARS DEAD 1 CAN-TAKE 1)>
        <ROOM FOREST-1 (VARS DEAD 1 CAN-TAKE 0)>
        <TREE 1 (VARS DEAD 0 CAN-TAKE 1)>
    )
*/

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectInstance {
    pub name: String,
    pub id: String,
    pub loc: ObjectLocation,
    pub vars: BTreeMap<String, i32>,
    pub nested: BTreeMap<String, Vec<String>>, // key: instance parent name, location must be ::Object(...)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ObjectLocation {
    Player,
    Room(String),
    // id is only valid if parent is an object instance, and is only filled out after nesting objects
    Object(String, usize, Option<String>), // parent name, index, id
}

pub struct ObjectActions {
    pub in_room: Option<String>, // when this object is in the same room as player, can be nested within an object in the same room (pass as PRSO)
    pub in_player: Option<String>, // when this object is in player inventory, ditto
    pub enter_player: Option<String>, // when this object enters player inventory, ditto
    pub exit_player: Option<String>, // when this object leaves player inventory, ditto
}

impl ObjectActions {
    pub fn new() -> ObjectActions {
        ObjectActions {
            in_room: None,
            in_player: None,
            enter_player: None,
            exit_player: None,
        }
    }
}

impl ObjectInfo {
    pub fn new() -> ObjectInfo {
        ObjectInfo {
            index: 0,
            name: String::new(),
            synonyms: BTreeSet::new(),
            desc: None,
            vars: BTreeMap::new(),
            copies: Vec::new(),
            actions: ObjectActions::new(),
        }
    }

    fn crunch_synonyms(node: &ZilNode) -> ValidationResult<BTreeSet<String>> {
        match num_children_more_than(node, 1) {
            Err(e) => return Err(vec![e]),
            _ => (),
        }

        let mut out: BTreeSet<String> = BTreeSet::new();
        for c in node.children.iter().skip(1) {
            let word = match get_token_as_word(c) {
                Ok(v) => v,
                Err(e) => {
                    return Err(vec![e]);
                }
            };
            out.insert(word);
        }

        Ok(out)
    }

    fn crunch_copies<F>(
        gen_inst_id: &mut F,
        node: &ZilNode,
    ) -> ValidationResult<Vec<ObjectInstance>>
    where
        F: FnMut() -> String,
    {
        let mut errors: Vec<String> = Vec::new();
        let mut out: Vec<ObjectInstance> = Vec::new();
        if let Err(e) = num_children_more_than(node, 1) {
            return Err(vec![e]);
        }

        for c in node.children.iter().skip(1) {
            if c.node_type != ZilNodeType::Cluster {
                errors.push(format!(
                    "Expected cluster, found \n{}",
                    format_file_location(&c)
                ));
                continue;
            }

            if let Err(e) = num_children_more_than(c, 0) {
                errors.push(e);
                continue;
            }

            let scope = match get_token_as_word(&c.children[0]) {
                Ok(v) => v,
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            };

            let mut find_vars = 0;
            let mut loc = ObjectLocation::Player;
            if scope == "ROOM" {
                if let Err(e) = num_children_between(c, 2, 3) {
                    errors.push(e);
                    continue;
                }
                let room = match get_token_as_word(&c.children[1]) {
                    Ok(v) => v,
                    Err(e) => {
                        errors.push(e);
                        continue;
                    }
                };
                loc = ObjectLocation::Room(room);
                if c.children.len() == 3 {
                    find_vars = 2;
                }
            } else if scope == "PLAYER" {
                // do nothing, loc is already player
                if let Err(e) = num_children_between(c, 1, 2) {
                    errors.push(e);
                    continue;
                }
                if c.children.len() == 2 {
                    find_vars = 1;
                }
            } else {
                if let Err(e) = num_children_between(c, 2, 3) {
                    errors.push(e);
                    continue;
                }

                let obj_index = match get_token_as_number(&c.children[1]) {
                    Ok(v) => v,
                    Err(e) => {
                        errors.push(e);
                        continue;
                    }
                };

                let obj_index = match i32_to_usize(obj_index) {
                    Ok(v) => v,
                    Err(e) => {
                        errors.push(e);
                        continue;
                    }
                };

                loc = ObjectLocation::Object(scope, obj_index, None);

                if c.children.len() == 3 {
                    find_vars = 2;
                }
            }

            let mut vars: BTreeMap<String, i32> = BTreeMap::new();
            if find_vars > 0 {
                vars = Helpers::crunch_vars(&c.children[find_vars])?;
            }

            // name and default vars are added later

            out.push(ObjectInstance {
                name: String::new(),
                id: gen_inst_id(),
                loc: loc,
                vars: vars,
                nested: BTreeMap::new(),
            });
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(out)
    }
}

impl ObjectStats {
    pub fn new() -> ObjectStats {
        ObjectStats {
            basis: Vec::new(),
            all_objects: BTreeMap::new(),
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
        // ObjectLocation(name of parent object, index of copy (aka instance) in parent object)
        // BTreeMap<name of child object, Vec<id> of child object instances>
        let mut objects_in_objects: HashMap<ObjectLocation, BTreeMap<String, Vec<String>>> =
            HashMap::new();

        for info in self.all_objects.values() {
            for copy in info.copies.iter() {
                match copy.loc {
                    ObjectLocation::Object(ref name, index, ..) => {
                        // add id to object location
                        let id = self.all_objects.get(name).unwrap().copies[index - 1]
                            .id
                            .clone();
                        let loc = ObjectLocation::Object(name.clone(), index, Some(id));

                        // save for later
                        let list = match objects_in_objects.get_mut(&loc) {
                            Some(v) => v,
                            None => {
                                let new_list: BTreeMap<String, Vec<String>> = BTreeMap::new();
                                objects_in_objects.insert(loc.clone(), new_list);
                                objects_in_objects.get_mut(&loc).unwrap()
                            }
                        };

                        list.insert(copy.name.clone(), vec![copy.id.clone()]);
                    }
                    _ => (),
                }
            }
        }

        for (parent, nested) in objects_in_objects.into_iter() {
            let (name, index, id) = match parent {
                ObjectLocation::Object(name, index, id) => (name, index, id),
                _ => unreachable!(),
            };

            // update all object instance locs with id
            for (child_name, child_ids) in nested.iter() {
                for child_id in child_ids.iter() {
                    let child = self.all_objects.get_mut(child_name).unwrap();
                    let child_copy = child.copies.iter_mut().find(|c| c.id == *child_id).unwrap();
                    child_copy.loc = ObjectLocation::Object(name.clone(), index, id.clone());
                }
            }

            // nest objects
            self.all_objects
                .get_mut(&name)
                .unwrap()
                .copies
                .get_mut(index - 1)
                .unwrap()
                .nested = nested;
        }
    }
}

impl Populator for ObjectStats {
    fn add_node(&mut self, node: ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> ValidationResult<()> {
        let mut errors: Vec<String> = Vec::new();
        let mut inst_id = 0;

        let mut next_inst_id = || -> String {
            inst_id += 1;
            format!("inst_{}", inst_id)
        };

        for (i, node) in self.basis.iter().enumerate() {
            let mut object_errors: Vec<String> = Vec::new();
            let mut info = ObjectInfo::new();

            if let Err(e) = num_children_more_than(node, 1) {
                errors.push(e);
                continue;
            }

            let object_name = match get_token_as_word(&node.children[1]) {
                Ok(v) => Some(v),
                Err(e) => {
                    object_errors.push(e);
                    None
                }
            };

            for c in node.children.iter().skip(2) {
                if c.node_type != ZilNodeType::Group {
                    object_errors.push(format!("Expected group\n{}", format_file_location(&c)));
                    continue;
                }

                if let Err(e) = num_children_more_than(c, 0) {
                    object_errors.push(e);
                    continue;
                }

                let group_name = match get_token_as_word(&c.children[0]) {
                    Ok(v) => Some(v),
                    Err(e) => {
                        object_errors.push(e);
                        continue;
                    }
                };
                let group_name = group_name.unwrap();

                match group_name.as_str() {
                    "AKA" => {
                        if info.synonyms.len() > 0 {
                            object_errors.push(format!(
                                "Duplicate synonyms node\n{}",
                                format_file_location(&c)
                            ));
                        } else {
                            match ObjectInfo::crunch_synonyms(&c) {
                                Ok(v) => info.synonyms = v,
                                Err(mut e) => object_errors.append(&mut e),
                            }
                        }
                    }
                    "COPY" => {
                        if info.copies.len() > 0 {
                            object_errors
                                .push(format!("Duplicate copy node\n{}", format_file_location(&c)));

                            // let huh = self.next_inst_id();
                        } else {
                            match ObjectInfo::crunch_copies(&mut next_inst_id, &c) {
                                Ok(v) => info.copies = v,
                                Err(mut e) => object_errors.append(&mut e),
                            }
                        }
                    }
                    "DESC" => {
                        if info.desc.is_some() {
                            object_errors
                                .push(format!("Duplicate desc node\n{}", format_file_location(&c)));
                        } else {
                            match Helpers::crunch_desc(&c) {
                                Ok(v) => info.desc = Some(v),
                                Err(mut e) => object_errors.append(&mut e),
                            }
                        }
                    }
                    "VARS" => {
                        if info.vars.len() > 0 {
                            object_errors
                                .push(format!("Duplicate vars node\n{}", format_file_location(&c)));
                        } else {
                            match Helpers::crunch_vars(&c) {
                                Ok(v) => info.vars = v,
                                Err(mut e) => object_errors.append(&mut e),
                            }
                        }
                    }
                    "ACT-IN-ROOM" => {
                        if info.actions.in_room.is_some() {
                            object_errors.push(format!(
                                "Duplicate in-room action node\n{}",
                                format_file_location(&c)
                            ));
                        } else {
                            match Helpers::crunch_action(&c) {
                                Ok(v) => info.actions.in_room = Some(v),
                                Err(mut e) => object_errors.append(&mut e),
                            }
                        }
                    }
                    "ACT-IN-PLAYER" => {
                        if info.actions.in_player.is_some() {
                            object_errors.push(format!(
                                "Duplicate in-player action node\n{}",
                                format_file_location(&c)
                            ));
                        } else {
                            match Helpers::crunch_action(&c) {
                                Ok(v) => info.actions.in_player = Some(v),
                                Err(mut e) => object_errors.append(&mut e),
                            }
                        }
                    }
                    "ACT-ADD" => {
                        if info.actions.enter_player.is_some() {
                            object_errors.push(format!(
                                "Duplicate enter-player action node\n{}",
                                format_file_location(&c)
                            ));
                        } else {
                            match Helpers::crunch_action(&c) {
                                Ok(v) => info.actions.enter_player = Some(v),
                                Err(mut e) => object_errors.append(&mut e),
                            }
                        }
                    }
                    "ACT-REMOVE" => {
                        if info.actions.exit_player.is_some() {
                            object_errors.push(format!(
                                "Duplicate exit-player action node\n{}",
                                format_file_location(&c)
                            ));
                        } else {
                            match Helpers::crunch_action(&c) {
                                Ok(v) => info.actions.exit_player = Some(v),
                                Err(mut e) => object_errors.append(&mut e),
                            }
                        }
                    }
                    _ => {
                        object_errors.push(format!(
                            "Object node has unknown group:{}\n{}",
                            group_name.clone(),
                            format_file_location(&c)
                        ));
                    }
                }
            }

            if object_errors.len() > 0 {
                errors.append(&mut object_errors);
                continue;
            }

            let object_name = object_name.unwrap();

            for copy in info.copies.iter_mut() {
                copy.name = object_name.clone();
                for (name, val) in info.vars.iter() {
                    if !copy.vars.contains_key(name) {
                        copy.vars.insert(name.clone(), *val);
                    }
                }
            }

            info.index = i;
            info.name = object_name.clone();
            match self.all_objects.insert(object_name.clone(), info) {
                Some(_) => {
                    errors.push(format!(
                        "Duplicate object name: {}\n{}",
                        object_name,
                        format_file_location(&node)
                    ));
                    continue;
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
        for key in self.all_objects.keys() {
            if CrossRef::name_is_illegal(key) {
                errors.push(format!("Illegal object name: {}", key));
            }
        }

        let routine_codex = cross_ref.routines.as_codex();
        let room_codex = cross_ref.rooms.as_codex();
        let object_codex = self.as_codex();

        for (key, info) in self.all_objects.iter() {
            for copies in info.copies.iter() {
                match copies.loc {
                    ObjectLocation::Player => (),
                    ObjectLocation::Room(ref room) => {
                        if room_codex.lookup(room).is_none() {
                            errors.push(format!(
                                "Object {} has invalid copy location room: {}",
                                key, room
                            ));
                        }
                    }
                    ObjectLocation::Object(ref obj, i, ..) => {
                        let parent = object_codex.lookup(obj);
                        if parent.is_none() {
                            errors.push(format!(
                                "Object {} has invalid copy location object name: {}",
                                key, obj
                            ));
                            continue;
                        }
                        let parent = parent.unwrap();

                        if parent.copies.len() < i {
                            errors.push(format!(
                                "Object {} has invalid copy location object ({}) index: {}, only has {} copies",
                                key, obj, i, parent.copies.len()
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
                                "Object {} has invalid desc routine: {}",
                                key, routine
                            ));
                        }
                    }
                    _ => (),
                }
            }

            if info.actions.in_room.is_some() {
                let action = info.actions.in_room.as_ref().unwrap();
                if routine_codex.lookup(action).is_none() {
                    errors.push(format!(
                        "Object {} has invalid in-room action routine: {}",
                        key, action
                    ));
                }
            }

            if info.actions.in_player.is_some() {
                let action = info.actions.in_player.as_ref().unwrap();
                if routine_codex.lookup(action).is_none() {
                    errors.push(format!(
                        "Object {} has invalid in-player action routine: {}",
                        key, action
                    ));
                }
            }

            if info.actions.enter_player.is_some() {
                let action = info.actions.enter_player.as_ref().unwrap();
                if routine_codex.lookup(action).is_none() {
                    errors.push(format!(
                        "Object {} has invalid add-player action routine: {}",
                        key, action
                    ));
                }
            }

            if info.actions.exit_player.is_some() {
                let action = info.actions.exit_player.as_ref().unwrap();
                if routine_codex.lookup(action).is_none() {
                    errors.push(format!(
                        "Object {} has invalid remove-player action routine: {}",
                        key, action
                    ));
                }
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(())
    }
}

pub struct ObjectCodex<'a> {
    index: usize,
    basis: &'a Vec<ZilNode>,
    all_objects: &'a BTreeMap<String, ObjectInfo>,
}
pub struct ObjectCodexValue<'a> {
    pub name: &'a String,
    pub synonyms: &'a BTreeSet<String>,
    pub desc: &'a Option<DescType>,
    #[allow(dead_code)]
    pub vars: &'a BTreeMap<String, i32>,
    pub copies: &'a Vec<ObjectInstance>,
    pub actions: &'a ObjectActions,
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
                desc: &info.desc,
                vars: &info.vars,
                copies: &info.copies,
                actions: &info.actions,
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
            desc: &info.desc,
            vars: &info.vars,
            copies: &info.copies,
            actions: &info.actions,
        });
    }
}
