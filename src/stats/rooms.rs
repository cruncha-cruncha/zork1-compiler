use std::collections::{HashMap, HashSet};

use crate::zil::node::{ZilNode, ZilNodeType};

use super::helpers::get_nth_child_name;

pub struct RoomCodex<'a> {
    lookup: &'a HashMap<String, &'a ZilNode>,
    pub subgroups: HashSet<String>,
}

impl<'a> RoomCodex<'a> {
    pub fn new(lookup: &'a HashMap<String, &ZilNode>) -> RoomCodex<'a> {
        RoomCodex {
            lookup,
            subgroups: HashSet::new(),
        }
    }

    pub fn populate(&mut self) {
        self.populate_subgroups();
    }

    fn populate_subgroups(&mut self) {
        for node in self.lookup.values() {
            for c in node.children.iter() {
                if c.node_type == ZilNodeType::Group {
                    match get_nth_child_name(0, c) {
                        Some(name) => {
                            self.subgroups.insert(name);
                        }
                        None => panic!("Group in room has no name"),
                    }
                }
            }
        }
    }
}

/*
IN     // direction, but also a word
LDESC
DESC
EAST   // direction
DOWN   // direction
SOUTH  // direction
WEST   // direction
FLAGS
UP     // direction
NORTH  // direction
ACTION
GLOBAL
SW     // direction
NW     // direction
PSEUDO
LAND   // direction
NE     // direction
SE     // direction
OUT    // direction
VALUE
*/