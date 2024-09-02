use std::collections::{HashMap, HashSet};

use crate::{
    stats::{
        cross_ref::Populator,
        helpers::{get_nth_child_as_word, get_token_as_number, get_token_as_word},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use crate::stats::cross_ref::Codex;

use super::syntax::ILLEGAL;

pub struct RoomStats<'a> {
    basis: HashMap<String, &'a ZilNode>,
    pub info: HashMap<String, RoomInfo<'a>>,
    pub groups: GroupCruncher,
}

pub struct GroupCruncher {
    pub direction_names: HashSet<String>, // all directions referenced by any room
    pub routines: HashSet<String>,        // all routines referenced by any room
    pub local_globals: HashSet<String>,   // all local-global objects referenced by any room
    pub objects: HashSet<String>,         // all objects referenced by any room
    pub rooms: HashSet<String>,           // all rooms referenced by any room
    pub globals: HashSet<String>,         // all globals referenced by any room
    pub flag_words: HashSet<String>,      // these don't need to correspond to anything
    pub pseudo_text: HashSet<String>, // not sure what these mean. Possibly fake objects? (like they take the place of objects in a SYNTAX)
}

pub struct RoomInfo<'a> {
    pub name: String,
    pub action: Option<String>,
    pub desc: Option<String>,
    pub ldesc: Option<String>,
    pub flags: HashSet<String>,
    pub pseudo: Vec<Pseudo>,
    pub globals: HashSet<String>,
    pub value: Option<i32>,
    pub directions: HashMap<String, Direction<'a>>,
}

pub struct Pseudo {
    pub name: String,
    pub routine: String,
}

pub struct Direction<'a> {
    pub name: String,
    pub kind: DirectionType,
    pub basis: &'a ZilNode,
}

#[derive(Clone, Copy)]
pub enum DirectionType {
    ZERO,  // SW <TEXT>
    ONE,   // SW PER <ROUTINE>
    TWO,   // SW TO <ROOM>
    THREE, // SW TO <ROOM> IF <GLOBAL>
    FOUR,  // SW TO <ROOM> IF <GLOBAL> ELSE <TEXT>
    FIVE,  // SW TO <ROOM> IF <OBJECT> IS OPEN
    SIX,   // SW TO <ROOM> IF <OBJECT> IS OPEN ELSE <TEXT>
}

impl<'a> RoomStats<'a> {
    pub fn new() -> RoomStats<'a> {
        RoomStats {
            basis: HashMap::new(),
            groups: GroupCruncher::new(),
            info: HashMap::new(),
        }
    }

    pub fn validate_direction_names(&self, directions: &impl Codex) -> Result<(), String> {
        for d in self.groups.direction_names.iter() {
            if directions.lookup(d).is_none() {
                return Err(format!("Direction {} not found", d));
            }
        }

        Ok(())
    }

    pub fn validate_routines(&self, routines: &impl Codex) -> Result<(), String> {
        for r in self.groups.routines.iter() {
            if routines.lookup(r).is_none() {
                return Err(format!("ROUTINE {} not found", r));
            }
        }

        Ok(())
    }

    pub fn validate_local_globals(&self, objects: &impl Codex) -> Result<(), String> {
        for o in self.groups.local_globals.iter() {
            match objects.lookup(o) {
                None => return Err(format!("local-global OBJECT {} not found", o)),
                Some(_) => {
                    // good enough
                }
            }
        }

        Ok(())
    }

    // TODO: there can be zero overlap between object names and room names?

    pub fn validate_objects(&self, objects: &impl Codex) -> Result<(), String> {
        for o in self.groups.objects.iter() {
            if objects.lookup(o).is_none() {
                return Err(format!("OBJECT {} not found", o));
            }
        }

        Ok(())
    }

    pub fn validate_rooms(&self) -> Result<(), String> {
        for r in self.groups.rooms.iter() {
            if self.lookup(r).is_none() {
                return Err(format!("ROOM {} not found", r));
            }
        }

        Ok(())
    }

    pub fn validate_globals(&self, globals: &impl Codex) -> Result<(), String> {
        for g in self.groups.globals.iter() {
            if globals.lookup(g).is_none() {
                return Err(format!("GLOBAL {} not found", g));
            }
        }

        Ok(())
    }
}

impl<'a> Populator<'a> for RoomStats<'a> {
    fn add_node(&mut self, node: &'a ZilNode) {
        let name = get_nth_child_as_word(1, node);
        match name {
            Some(name) => {
                if ILLEGAL.is_match(&name) {
                    panic!("Room node has illegal name {}", &name);
                }

                if self.basis.insert(name, node).is_some() {
                    panic!(
                        "Room node has duplicate name {}",
                        get_nth_child_as_word(1, node).unwrap()
                    );
                }
            }
            None => panic!("Room node has no name\n{}", format_file_location(&node)),
        }
    }

    fn crunch(&mut self) -> Result<(), String> {
        let mut group_cruncher = GroupCruncher::new();

        for n in self.basis.values() {
            match group_cruncher.munch(n) {
                Ok(info) => {
                    self.info.insert(info.name.clone(), info);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        self.groups = group_cruncher;

        Ok(())
    }

    fn validate(&self, cross_ref: &crate::stats::cross_ref::CrossRef) -> Result<(), String> {
        self.validate_direction_names(&cross_ref.directions)?;
        self.validate_objects(&cross_ref.objects)?;
        self.validate_local_globals(&cross_ref.objects)?;
        self.validate_globals(&cross_ref.globals)?;
        self.validate_routines(&cross_ref.routines)?;
        self.validate_rooms()?;

        Ok(())
    }
}

impl<'a> Codex for RoomStats<'a> {
    fn lookup(&self, word: &str) -> Option<&ZilNode> {
        self.basis.get(word).map(|n| *n)
    }
}

impl<'a> GroupCruncher {
    pub fn new() -> GroupCruncher {
        GroupCruncher {
            direction_names: HashSet::new(),
            routines: HashSet::new(),
            local_globals: HashSet::new(),
            rooms: HashSet::new(),
            globals: HashSet::new(),
            flag_words: HashSet::new(),
            pseudo_text: HashSet::new(),
            objects: HashSet::new(),
        }
    }

    pub fn munch(&mut self, node: &'a ZilNode) -> Result<RoomInfo<'a>, String> {
        let mut out = RoomInfo::new();

        if node.children.len() < 2 {
            return Err(format!(
                "Room node has less than two children\n{}",
                format_file_location(&node)
            ));
        }

        match get_nth_child_as_word(1, node) {
            Some(name) => out.name = name,
            None => {
                return Err(format!(
                    "Room node has no name\n{}",
                    format_file_location(&node)
                ))
            }
        }

        for c in node.children.iter().skip(2) {
            if c.node_type != ZilNodeType::Group {
                return Err(format!(
                    "Room node has non-group child\n{}",
                    format_file_location(&node)
                ));
            }

            let name = match get_nth_child_as_word(0, c) {
                Some(name) => name,
                None => {
                    return Err(format!(
                        "Group in room has no name\n{}",
                        format_file_location(&node)
                    ))
                }
            };

            match name.as_str() {
                "ACTION" => {
                    if c.children.len() != 2 {
                        return Err(format!(
                            "ACTION group in room doesn't have two children\n{}",
                            format_file_location(&node)
                        ));
                    }

                    match get_nth_child_as_word(1, c) {
                        Some(name) => {
                            if out.action.is_some() {
                                return Err(format!(
                                    "Room has multiple ACTION groups\n{}",
                                    format_file_location(&node)
                                ));
                            }

                            out.action = Some(name.clone());
                            self.routines.insert(name);
                        }
                        None => {
                            return Err(format!(
                                "ACTION group in room doesn't have a named routine\n{}",
                                format_file_location(&node)
                            ));
                        }
                    }
                }
                "DESC" | "LDESC" => {
                    if c.children.len() != 2 {
                        return Err(format!(
                            "{} group in room doesn't have two children\n{}",
                            name,
                            format_file_location(&node)
                        ));
                    }

                    if c.children[1].node_type != ZilNodeType::Token(TokenType::Text) {
                        return Err(format!(
                            "{} group in room doesn't have a text type second child\n{}",
                            name,
                            format_file_location(&node)
                        ));
                    }

                    if name == "DESC" {
                        if out.desc.is_some() {
                            return Err(format!(
                                "Room has multiple DESC groups\n{}",
                                format_file_location(&node)
                            ));
                        }
                        out.desc = Some(c.children[1].get_first_token().unwrap().value.clone());
                    } else {
                        if out.ldesc.is_some() {
                            return Err(format!(
                                "Room has multiple FDESC groups\n{}",
                                format_file_location(&node)
                            ));
                        }
                        out.ldesc = Some(c.children[1].get_first_token().unwrap().value.clone());
                    }
                }
                "FLAGS" => {
                    if c.children.len() < 2 {
                        return Err(format!(
                            "FLAGS group in room doesn't have enough children\n{}",
                            format_file_location(&node)
                        ));
                    }

                    if out.flags.len() > 0 {
                        return Err(format!(
                            "Room has multiple FLAGS groups\n{}",
                            format_file_location(&node)
                        ));
                    }

                    for c in c.children.iter().skip(1) {
                        match get_token_as_word(c) {
                            Some(name) => {
                                out.flags.insert(name.clone());
                                self.flag_words.insert(name);
                            }
                            None => {
                                return Err(format!(
                                    "FLAGS group in room has a child that isn't a word\n{}",
                                    format_file_location(&node)
                                ));
                            }
                        }
                    }
                }
                "GLOBAL" => {
                    if c.children.len() < 2 {
                        return Err(format!(
                            "GLOBAL group in room doesn't have enough children\n{}",
                            format_file_location(&node)
                        ));
                    }

                    if out.globals.len() > 0 {
                        return Err(format!(
                            "Room has multiple GLOBAL groups\n{}",
                            format_file_location(&node)
                        ));
                    }

                    for c in c.children.iter().skip(1) {
                        match get_token_as_word(c) {
                            Some(name) => {
                                out.globals.insert(name.clone());
                                self.local_globals.insert(name);
                            }
                            None => {
                                return Err(format!(
                                    "GLOBAL group in room has a child that isn't a word\n{}",
                                    format_file_location(&node)
                                ));
                            }
                        }
                    }
                }
                "PSEUDO" => {
                    if c.children.len() < 2 || c.children.len() % 2 == 0 {
                        return Err(format!(
                            "PSEUDO group in room has too few or even number of children\n{}",
                            format_file_location(&node)
                        ));
                    }

                    if out.pseudo.len() > 0 {
                        return Err(format!(
                            "Room has multiple PSEUDO groups\n{}",
                            format_file_location(&node)
                        ));
                    }

                    let mut pseudo_name = String::new();
                    for (i, c) in c.children.iter().skip(1).enumerate() {
                        if i % 2 == 0 {
                            if c.node_type != ZilNodeType::Token(TokenType::Text) {
                                return Err(format!(
                                    "PSEUDO group in room has an even child that isn't text\n{}",
                                    format_file_location(&node)
                                ));
                            }

                            let val = c.get_first_token().unwrap().value.clone();
                            self.pseudo_text.insert(val.clone());
                            pseudo_name = val;
                        } else {
                            match get_token_as_word(c) {
                                Some(name) => {
                                    self.routines.insert(name.clone());
                                    out.pseudo.push(Pseudo {
                                        name: pseudo_name.clone(),
                                        routine: name,
                                    })
                                }
                                None => {
                                    return Err(format!(
                                    "PSEUDO group in room has an odd child that isn't a word\n{}",
                                    format_file_location(&node)
                                ));
                                }
                            }
                        }
                    }
                }
                "VALUE" => {
                    if c.children.len() != 2 {
                        return Err(format!(
                            "VALUE group in room doesn't have two children\n{}",
                            format_file_location(&node)
                        ));
                    }

                    if c.children[1].node_type != ZilNodeType::Token(TokenType::Number) {
                        return Err(format!(
                            "VALUE group in room doesn't have a number type second child\n{}",
                            format_file_location(&node)
                        ));
                    }

                    if out.value.is_some() {
                        return Err(format!(
                            "Room has multiple VALUE groups\n{}",
                            format_file_location(&node)
                        ));
                    }

                    let val = get_token_as_number(&c.children[1]).unwrap();
                    out.value = Some(val);
                }
                _ => match self.parse_direction(c) {
                    Some(dir) => {
                        if out.directions.contains_key(&dir.name) {
                            return Err(format!(
                                "Room has duplicate directions\n{}",
                                format_file_location(&node)
                            ));
                        }

                        self.direction_names.insert(dir.name.clone());
                        out.directions.insert(dir.name.clone(), dir);
                    }
                    None => {
                        return Err(format!(
                            "Room has bad group {}\n{}",
                            name,
                            format_file_location(&node)
                        ));
                    }
                },
            }
        }

        Ok(out)
    }

    pub fn parse_direction(&mut self, node: &'a ZilNode) -> Option<Direction<'a>> {
        if node.children.len() < 2 {
            return None;
        }

        let name = match get_nth_child_as_word(0, node) {
            Some(name) => name,
            None => {
                return None;
            }
        };

        let mut out = Direction {
            basis: node,
            name: name,
            kind: DirectionType::ZERO,
        };

        if node.children.len() == 2 {
            if node.children[1].node_type != ZilNodeType::Token(TokenType::Text) {
                return None;
            }

            out.kind = DirectionType::ZERO;
            return Some(out);
        }

        if node.children.len() < 3 {
            return None;
        }

        match get_nth_child_as_word(1, node) {
            Some(name) => {
                if name == "PER" {
                    if node.children.len() == 3
                        && node.children[2].node_type == ZilNodeType::Token(TokenType::Word)
                    {
                        self.routines
                            .insert(get_nth_child_as_word(2, node).unwrap());
                        out.kind = DirectionType::ONE;
                        return Some(out);
                    }
                    return None;
                } else if name != "TO" {
                    return None;
                }
            }
            None => {
                return None;
            }
        }

        if node.children[2].node_type != ZilNodeType::Token(TokenType::Word) {
            return None;
        }

        self.rooms.insert(get_nth_child_as_word(2, node).unwrap());

        if node.children.len() == 3 {
            out.kind = DirectionType::TWO;
            return Some(out);
        }

        if node.children.len() < 5 {
            return None;
        }

        if node.children[4].node_type != ZilNodeType::Token(TokenType::Word) {
            return None;
        }

        if node.children.len() == 5 {
            self.globals.insert(get_nth_child_as_word(4, node).unwrap());
            out.kind = DirectionType::THREE;
            return Some(out);
        }

        if node.children.len() < 7 {
            return None;
        }

        match get_nth_child_as_word(5, node) {
            Some(name) => {
                if name == "ELSE" {
                    if node.children.len() == 7
                        && node.children[6].node_type == ZilNodeType::Token(TokenType::Text)
                    {
                        self.globals.insert(get_nth_child_as_word(4, node).unwrap());
                        out.kind = DirectionType::FOUR;
                        return Some(out);
                    }
                    return None;
                } else if name != "IS" {
                    return None;
                }
            }
            None => {
                return None;
            }
        }

        match get_nth_child_as_word(6, node) {
            Some(name) => {
                if name != "OPEN" {
                    return None;
                }
            }
            None => {
                return None;
            }
        }

        self.objects.insert(get_nth_child_as_word(4, node).unwrap());

        if node.children.len() == 7 {
            out.kind = DirectionType::FIVE;
            return Some(out);
        }

        if node.children.len() < 9 {
            return None;
        }

        match get_nth_child_as_word(7, node) {
            Some(name) => {
                if name == "ELSE" {
                    if node.children.len() == 9
                        && node.children[8].node_type == ZilNodeType::Token(TokenType::Text)
                    {
                        out.kind = DirectionType::SIX;
                        return Some(out);
                    }
                }
            }
            None => (),
        }

        None
    }
}

impl<'a> RoomInfo<'a> {
    pub fn new() -> RoomInfo<'a> {
        RoomInfo {
            name: String::new(),
            action: None,
            desc: None,
            ldesc: None,
            flags: HashSet::new(),
            pseudo: Vec::new(),
            globals: HashSet::new(),
            value: None,
            directions: HashMap::new(),
        }
    }
}
