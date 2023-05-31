use std::collections::{HashMap, HashSet};

use crate::{
    stats::helpers::get_bunch_name,
    zil::{
        file_table::format_file_location,
        node::{TokenBunchType, ZilNode, ZilNodeType},
    },
};

use crate::stats::{cross_ref::Codex, helpers::get_nth_child_name};

pub struct RoomCodex<'a> {
    basis: HashMap<String, &'a ZilNode>,
    info: HashMap<String, RoomInfo<'a>>,
    pub groups: GroupCruncher,
}

pub struct GroupCruncher {
    pub direction_names: HashSet<String>, // all directions referenced by any room
    pub routines: HashSet<String>,        // all routines referenced by any room
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
    pub pseudo: Option<&'a ZilNode>, // TODO
    pub globals: HashSet<String>,
    pub value: Option<usize>,
    pub directions: HashMap<String, Direction<'a>>,
}

pub struct Direction<'a> {
    pub name: String,
    pub kind: DirectionType,
    pub basis: &'a ZilNode,
}

pub enum DirectionType {
    ZERO,  // SW <TEXT>
    ONE,   // SW PER <ROUTINE>
    TWO,   // SW TO <ROOM>
    THREE, // SW TO <ROOM> IF <GLOBAL>
    FOUR,  // SW TO <ROOM> IF <GLOBAL> ELSE <TEXT>
    FIVE,  // SW TO <ROOM> IF <OBJECT> IS OPEN
    SIX,   // SW TO <ROOM> IF <OBJECT> IS OPEN ELSE <TEXT>
}

impl<'a> RoomCodex<'a> {
    pub fn new() -> RoomCodex<'a> {
        RoomCodex {
            basis: HashMap::new(),
            groups: GroupCruncher::new(),
            info: HashMap::new(),
        }
    }

    pub fn get_info(&self, word: &str) -> Option<&RoomInfo> {
        self.info.get(word)
    }

    pub fn validate_direction_names(&self, directions: &impl Codex<'a>) -> Result<(), String> {
        for d in self.groups.direction_names.iter() {
            if directions.lookup(d).is_none() {
                return Err(format!("Direction {} not found", d));
            }
        }

        Ok(())
    }

    pub fn validate_routines(&self, routines: &impl Codex<'a>) -> Result<(), String> {
        for r in self.groups.routines.iter() {
            if routines.lookup(r).is_none() {
                return Err(format!("ROUTINE {} not found", r));
            }
        }

        Ok(())
    }

    pub fn validate_objects(&self, objects: &impl Codex<'a>) -> Result<(), String> {
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

    pub fn validate_globals(&self, globals: &impl Codex<'a>) -> Result<(), String> {
        for g in self.groups.globals.iter() {
            if globals.lookup(g).is_none() {
                return Err(format!("GLOBAL {} not found", g));
            }
        }

        Ok(())
    }
}

impl<'a> Codex<'a> for RoomCodex<'a> {
    fn get_name(&self) -> String {
        String::from("rooms")
    }

    fn add_node(&mut self, node: &'a ZilNode) {
        let name = get_nth_child_name(1, node);
        match name {
            Some(name) => {
                if self.basis.insert(name, node).is_some() {
                    panic!(
                        "Room node has duplicate name {}",
                        get_nth_child_name(1, node).unwrap()
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

    fn lookup(&self, word: &str) -> Option<&ZilNode> {
        self.basis.get(word).map(|n| *n)
    }

    fn into_iter(&self) -> std::vec::IntoIter<String> {
        self.basis
            .keys()
            .map(|k| k.clone())
            .collect::<Vec<String>>()
            .into_iter()
    }
}

impl<'a> GroupCruncher {
    pub fn new() -> GroupCruncher {
        GroupCruncher {
            direction_names: HashSet::new(),
            routines: HashSet::new(),
            objects: HashSet::new(),
            rooms: HashSet::new(),
            globals: HashSet::new(),
            flag_words: HashSet::new(),
            pseudo_text: HashSet::new(),
        }
    }

    pub fn munch(&mut self, node: &'a ZilNode) -> Result<RoomInfo<'a>, String> {
        let mut out = RoomInfo::new();
        for c in node.children.iter().skip(2) {
            if c.node_type != ZilNodeType::Group {
                return Err(format!(
                    "Room node has non-group child\n{}",
                    format_file_location(&node)
                ));
            }

            let name = match get_nth_child_name(0, c) {
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

                    match get_nth_child_name(1, c) {
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

                    if c.children[1].node_type != ZilNodeType::TokenBunch(TokenBunchType::Text) {
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
                        match get_bunch_name(c) {
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
                        match get_bunch_name(c) {
                            Some(name) => {
                                out.globals.insert(name.clone());
                                self.objects.insert(name);
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

                    if out.pseudo.is_some() {
                        return Err(format!(
                            "Room has multiple PSEUDO groups\n{}",
                            format_file_location(&node)
                        ));
                    }

                    out.pseudo = Some(c);

                    for (i, c) in c.children.iter().skip(1).enumerate() {
                        if i % 2 == 0 {
                            if c.node_type != ZilNodeType::TokenBunch(TokenBunchType::Text) {
                                return Err(format!(
                                    "PSEUDO group in room has an even child that isn't text\n{}",
                                    format_file_location(&node)
                                ));
                            }

                            let val = c.get_first_token().unwrap().value.clone();
                            self.pseudo_text.insert(val);
                        } else {
                            match get_bunch_name(c) {
                                Some(name) => {
                                    self.routines.insert(name);
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

                    if c.children[1].node_type != ZilNodeType::TokenBunch(TokenBunchType::Number) {
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

                    let val = c.children[1].get_first_token().unwrap().value.clone();
                    out.value = Some(val.parse::<usize>().unwrap());
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

        let name = match get_nth_child_name(0, node) {
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
            if node.children[1].node_type != ZilNodeType::TokenBunch(TokenBunchType::Text) {
                return None;
            }

            out.kind = DirectionType::ZERO;
            return Some(out);
        }

        if node.children.len() < 3 {
            return None;
        }

        match get_nth_child_name(1, node) {
            Some(name) => {
                if name == "PER" {
                    if node.children.len() == 3
                        && node.children[2].node_type
                            == ZilNodeType::TokenBunch(TokenBunchType::Word)
                    {
                        self.routines.insert(get_nth_child_name(2, node).unwrap());
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

        if node.children[2].node_type != ZilNodeType::TokenBunch(TokenBunchType::Word) {
            return None;
        }

        self.rooms.insert(get_nth_child_name(2, node).unwrap());

        if node.children.len() == 3 {
            out.kind = DirectionType::TWO;
            return Some(out);
        }

        if node.children.len() < 5 {
            return None;
        }

        if node.children[4].node_type != ZilNodeType::TokenBunch(TokenBunchType::Word) {
            return None;
        }

        if node.children.len() == 5 {
            self.globals.insert(get_nth_child_name(4, node).unwrap());
            out.kind = DirectionType::THREE;
            return Some(out);
        }

        if node.children.len() < 7 {
            return None;
        }

        match get_nth_child_name(5, node) {
            Some(name) => {
                if name == "ELSE" {
                    if node.children.len() == 7
                        && node.children[6].node_type
                            == ZilNodeType::TokenBunch(TokenBunchType::Text)
                    {
                        self.globals.insert(get_nth_child_name(4, node).unwrap());
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

        match get_nth_child_name(6, node) {
            Some(name) => {
                if name != "OPEN" {
                    return None;
                }
            }
            None => {
                return None;
            }
        }

        self.objects.insert(get_nth_child_name(4, node).unwrap());

        if node.children.len() == 7 {
            out.kind = DirectionType::FIVE;
            return Some(out);
        }

        if node.children.len() < 9 {
            return None;
        }

        match get_nth_child_name(7, node) {
            Some(name) => {
                if name == "ELSE" {
                    if node.children.len() == 9
                        && node.children[8].node_type
                            == ZilNodeType::TokenBunch(TokenBunchType::Text)
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
            pseudo: None,
            globals: HashSet::new(),
            value: None,
            directions: HashMap::new(),
        }
    }
}
