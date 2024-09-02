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

pub const UNDEFINED_LOC: &str = "NOWHERE";

pub struct ObjectStats {
    basis: HashMap<String, ZilNode>,
    pub info: HashMap<String, ObjectInfo>,
    pub groups: GroupCruncher,
}

pub struct GroupCruncher {
    pub routines: HashSet<String>,
    pub room_or_object: HashSet<String>,
    pub objects: HashSet<String>,
    pub adjectives: HashSet<String>,
    pub synonyms: HashSet<String>,
    pub flag_words: HashSet<String>, // these don't need to correspond to anything
    pub vtypes: HashSet<String>,     // idk
}

pub struct ObjectInfo {
    pub name: String,
    pub loc: Option<String>,
    pub adjectives: HashSet<String>,
    pub synonyms: HashSet<String>,
    pub desc_fcn: Option<String>,
    pub action: Option<String>,
    pub text: Option<String>,
    pub desc: Option<String>,
    pub ldesc: Option<String>,
    pub fdesc: Option<String>,
    pub flags: HashSet<String>,
    pub capacity: Option<i32>,
    pub size: Option<i32>,
    pub strength: Option<i32>,
    pub value: Option<i32>,
    pub tvalue: Option<i32>,
    pub vtype: Option<String>,
}

impl ObjectStats {
    pub fn new() -> ObjectStats {
        ObjectStats {
            basis: HashMap::new(),
            info: HashMap::new(),
            groups: GroupCruncher::new(),
        }
    }

    pub fn validate_routines(&self, routines: &impl Codex) -> Result<(), String> {
        for r in self.groups.routines.iter() {
            if routines.lookup(r).is_none() {
                return Err(format!("ROUTINE {} not found", r));
            }
        }

        Ok(())
    }

    pub fn validate_room_or_object(&self, rooms: &impl Codex) -> Result<(), String> {
        for roo in self.groups.room_or_object.iter() {
            if self.lookup(roo).is_none() {
                if rooms.lookup(roo).is_none() {
                    return Err(format!("ROOM or OBJECT {} not found", roo));
                }
            }
        }

        Ok(())
    }

    pub fn validate_objects(&self) -> Result<(), String> {
        for o in self.groups.objects.iter() {
            if self.lookup(o).is_none() {
                return Err(format!("OBJECT {} not found", o));
            }
        }

        Ok(())
    }
}

impl Populator for ObjectStats {
    fn add_node(&mut self, node: ZilNode) {
        let name = get_nth_child_as_word(1, &node);
        match name {
            Some(name) => {
                if ILLEGAL.is_match(&name) {
                    panic!("Object node has illegal name {}", &name);
                }

                if self.basis.insert(name.clone(), node).is_some() {
                    panic!("Object node has duplicate name {}", name);
                }
            }
            None => panic!("Object node has no name\n{}", format_file_location(&node)),
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
        self.validate_routines(&cross_ref.routines)?;
        self.validate_room_or_object(&cross_ref.rooms)?;
        self.validate_objects()?;

        Ok(())
    }
}

impl Codex for ObjectStats {
    fn lookup(&self, word: &str) -> Option<&ZilNode> {
        self.basis.get(word)
    }
}

impl<'a> GroupCruncher {
    pub fn new() -> GroupCruncher {
        GroupCruncher {
            routines: HashSet::new(),
            room_or_object: HashSet::new(),
            flag_words: HashSet::new(),
            adjectives: HashSet::new(),
            synonyms: HashSet::new(),
            vtypes: HashSet::new(),
            objects: HashSet::new(),
        }
    }

    pub fn munch(&mut self, node: &'a ZilNode) -> Result<ObjectInfo, String> {
        let mut out = ObjectInfo::new();

        if node.children.len() < 2 {
            return Err(format!(
                "Object node has less than two children\n{}",
                format_file_location(&node)
            ));
        }

        match get_nth_child_as_word(1, node) {
            Some(name) => out.name = name,
            None => {
                return Err(format!(
                    "Object node has no name\n{}",
                    format_file_location(&node)
                ))
            }
        }

        for c in node.children.iter().skip(2) {
            if c.node_type != ZilNodeType::Group {
                return Err(format!(
                    "Object node has non-group child\n{}",
                    format_file_location(&node)
                ));
            }

            let name = match get_nth_child_as_word(0, c) {
                Some(name) => name,
                None => {
                    return Err(format!(
                        "Group in object has no name\n{}",
                        format_file_location(&node)
                    ))
                }
            };

            match name.as_str() {
                "ACTION" => {
                    if c.children.len() != 2 {
                        return Err(format!(
                            "ACTION group in object doesn't have two children\n{}",
                            format_file_location(&node)
                        ));
                    }

                    match get_nth_child_as_word(1, c) {
                        Some(name) => {
                            if out.action.is_some() {
                                return Err(format!(
                                    "Object has multiple ACTION groups\n{}",
                                    format_file_location(&node)
                                ));
                            }

                            out.action = Some(name.clone());
                            self.routines.insert(name);
                        }
                        None => {
                            return Err(format!(
                                "ACTION group in object doesn't have a named routine\n{}",
                                format_file_location(&node)
                            ));
                        }
                    }
                }
                "ADJECTIVE" => {
                    if c.children.len() < 2 {
                        return Err(format!(
                            "ADJECTIVE group in object doesn't have enough children\n{}",
                            format_file_location(&node)
                        ));
                    }

                    if out.adjectives.len() > 0 {
                        return Err(format!(
                            "Object has multiple ADJECTIVE groups\n{}",
                            format_file_location(&node)
                        ));
                    }

                    for child in c.children.iter().skip(1) {
                        if child.node_type != ZilNodeType::Token(TokenType::Word) {
                            return Err(format!(
                                "ADJECTIVE group in object has non-word child\n{}",
                                format_file_location(&node)
                            ));
                        }

                        let val = get_token_as_word(child).unwrap();
                        out.adjectives.insert(val.clone());
                        self.adjectives.insert(val);
                    }
                }
                "SYNONYM" => {
                    if c.children.len() < 2 {
                        return Err(format!(
                            "SYNONYM group in object doesn't have enough children\n{}",
                            format_file_location(&node)
                        ));
                    }

                    if out.adjectives.len() > 0 {
                        return Err(format!(
                            "Object has multiple SYNONYM groups\n{}",
                            format_file_location(&node)
                        ));
                    }

                    for child in c.children.iter().skip(1) {
                        if child.node_type != ZilNodeType::Token(TokenType::Word) {
                            return Err(format!(
                                "SYNONYM group in object has non-word child\n{}",
                                format_file_location(&node)
                            ));
                        }

                        let val = get_token_as_word(child).unwrap();
                        out.synonyms.insert(val.clone());
                        self.synonyms.insert(val);
                    }
                }
                "TEXT" | "DESC" | "LDESC" | "FDESC" => {
                    if c.children.len() != 2 {
                        return Err(format!(
                            "{} group in object doesn't have two children\n{}",
                            name,
                            format_file_location(&node)
                        ));
                    }

                    if c.children[1].node_type != ZilNodeType::Token(TokenType::Text) {
                        return Err(format!(
                            "{} group in object doesn't have a text type second child\n{}",
                            name,
                            format_file_location(&node)
                        ));
                    }

                    let get_err = || {
                        Err(format!(
                            "Object has multiple {} groups\n{}",
                            name,
                            format_file_location(&node)
                        ))
                    };

                    let val = c.children[1].get_first_token().unwrap().value.clone();

                    match name.as_str() {
                        "TEXT" => {
                            if out.text.is_some() {
                                return get_err();
                            }
                            out.text = Some(val);
                        }
                        "DESC" => {
                            if out.desc.is_some() {
                                return get_err();
                            }
                            out.desc = Some(val);
                        }
                        "LDESC" => {
                            if out.ldesc.is_some() {
                                return get_err();
                            }
                            out.ldesc = Some(val);
                        }
                        "FDESC" => {
                            if out.fdesc.is_some() {
                                return get_err();
                            }
                            out.fdesc = Some(val);
                        }
                        _ => unreachable!(),
                    }
                }
                "DESCFCN" => {
                    if c.children.len() != 2 {
                        return Err(format!(
                            "DESCFCN group in object doesn't have two children\n{}",
                            format_file_location(&node)
                        ));
                    }

                    if c.children[1].node_type != ZilNodeType::Token(TokenType::Word) {
                        return Err(format!(
                            "DESCFCN group in object doesn't have a word type second child\n{}",
                            format_file_location(&node)
                        ));
                    }

                    if out.desc_fcn.is_some() {
                        return Err(format!(
                            "Object has multiple DESCFCN groups\n{}",
                            format_file_location(&node)
                        ));
                    }

                    let val = get_token_as_word(&c.children[1]).unwrap();
                    out.desc_fcn = Some(val.clone());
                    self.routines.insert(val);
                }
                "FLAGS" => {
                    if c.children.len() < 2 {
                        return Err(format!(
                            "FLAGS group in object doesn't have enough children\n{}",
                            format_file_location(&node)
                        ));
                    }

                    if out.flags.len() > 0 {
                        return Err(format!(
                            "Object has multiple FLAGS groups\n{}",
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
                                    "FLAGS group in object has a child that isn't a word\n{}",
                                    format_file_location(&node)
                                ));
                            }
                        }
                    }
                }
                "IN" => {
                    if c.children.len() == 2
                        && c.children[1].node_type == ZilNodeType::Token(TokenType::Word)
                    {
                        let name = get_nth_child_as_word(1, c).unwrap();
                        if out.loc.is_some() {
                            return Err(format!(
                                "Object has multiple IN groups\n{}",
                                format_file_location(&node)
                            ));
                        }

                        out.loc = Some(name.clone());
                        self.room_or_object.insert(name);
                    } else {
                        return Err(format!(
                            "IN group in object doesn't have a single word child\n{}",
                            format_file_location(&node)
                        ));
                    }
                }
                "CAPACITY" | "SIZE" | "STRENGTH" | "VALUE" | "TVALUE" => {
                    if c.children.len() != 2 {
                        return Err(format!(
                            "{} group in object doesn't have two children\n{}",
                            name,
                            format_file_location(&node)
                        ));
                    }

                    if c.children[1].node_type != ZilNodeType::Token(TokenType::Number) {
                        return Err(format!(
                            "{} group in room doesn't have a number type second child\n{}",
                            name,
                            format_file_location(&node)
                        ));
                    }

                    let get_err = || {
                        Err(format!(
                            "Object has multiple {} groups\n{}",
                            name,
                            format_file_location(&node)
                        ))
                    };

                    let val = get_token_as_number(&c.children[1]).unwrap();

                    match name.as_str() {
                        "CAPACITY" => {
                            if out.capacity.is_some() {
                                return get_err();
                            }
                            out.capacity = Some(val);
                        }
                        "SIZE" => {
                            if out.size.is_some() {
                                return get_err();
                            }
                            out.size = Some(val);
                        }
                        "STRENGTH" => {
                            if out.strength.is_some() {
                                return get_err();
                            }
                            out.strength = Some(val);
                        }
                        "VALUE" => {
                            if out.value.is_some() {
                                return get_err();
                            }
                            out.value = Some(val);
                        }
                        "TVALUE" => {
                            if out.tvalue.is_some() {
                                return get_err();
                            }
                            out.tvalue = Some(val);
                        }
                        _ => unreachable!(),
                    }
                }
                "VTYPE" => {
                    if c.children.len() < 2 {
                        return Err(format!(
                            "VTYPE group in object doesn't have enough children\n{}",
                            format_file_location(&node)
                        ));
                    }

                    for c in c.children.iter().skip(1) {
                        match get_token_as_word(c) {
                            Some(name) => {
                                out.vtype = Some(name.clone());
                                self.vtypes.insert(name);
                            }
                            None => {
                                return Err(format!(
                                    "VTYPE group in object has a child that isn't a word\n{}",
                                    format_file_location(&node)
                                ));
                            }
                        }
                    }
                }
                _ => {
                    return Err(format!(
                        "Object has bad group {}\n{}",
                        name,
                        format_file_location(&node)
                    ));
                }
            }
        }

        if out.loc.is_none() {
            out.loc = Some(String::from(UNDEFINED_LOC));
        }

        Ok(out)
    }
}

impl<'a> ObjectInfo {
    pub fn new() -> ObjectInfo {
        ObjectInfo {
            name: String::new(),
            text: None,
            desc: None,
            ldesc: None,
            fdesc: None,
            desc_fcn: None,
            flags: HashSet::new(),
            loc: None,
            capacity: None,
            size: None,
            strength: None,
            value: None,
            tvalue: None,
            adjectives: HashSet::new(),
            synonyms: HashSet::new(),
            action: None,
            vtype: None,
        }
    }
}
