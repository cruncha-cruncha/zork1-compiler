use std::collections::{HashMap, HashSet};

use crate::zil::node::{ZilNode, ZilNodeType};

use super::{helpers::get_nth_child_name, top_level::Codex};

pub struct RoomCodex<'a> {
    basis: HashMap<String, &'a ZilNode>,
    pub subgroup_names: HashSet<String>,
}

/*
NORTH
LDESC
ACTION
DESC
SW
SE
OUT
VALUE
EAST
SOUTH
NW
IN
WEST
DOWN
FLAGS
GLOBAL
UP
LAND
PSEUDO
NE
*/

impl<'a> RoomCodex<'a> {
    pub fn new() -> RoomCodex<'a> {
        RoomCodex {
            basis: HashMap::new(),
            subgroup_names: HashSet::new(),
        }
    }
}

impl<'a> IntoIterator for RoomCodex<'a> {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.basis.keys().map(|k| k.clone()).collect::<Vec<String>>().into_iter()
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
                    panic!("Room node has duplicate name {}", get_nth_child_name(1, node).unwrap());
                }
            }
            None => panic!("Room node has no name"),
        }
    }

    fn crunch(&mut self) -> Result<(), String> {
        for n in self.basis.values() {
            for c in n.children.iter().skip(2) {
                if c.node_type != ZilNodeType::Group {
                    return Err(String::from("Room node has non-group child"));
                }

                match get_nth_child_name(0, c) {
                    Some(name) => {
                        self.subgroup_names.insert(name);
                    }
                    None => return Err(String::from("Group in room has no name")),
                }
            }
        }

        Ok(())
    }

    fn lookup(&self, word: &str) -> Option<&ZilNode> {
        self.basis.get(word).map(|n| *n)
    }
}