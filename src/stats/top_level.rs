use std::collections::HashMap;

use crate::zil::{
    ast::Tree,
    node::{ZilNode, ZilNodeType},
};

use super::helpers::get_nth_child_name;

pub struct CrossRef<'a> {
    tree: &'a Tree,
    pub rooms: HashMap<String, &'a ZilNode>,
    pub objects: HashMap<String, &'a ZilNode>,
    pub directions: Option<&'a ZilNode>,
    pub routines: HashMap<String, &'a ZilNode>,
    pub globals: HashMap<String, &'a ZilNode>,
    pub gdecls: HashMap<String, &'a ZilNode>,
    pub constants: HashMap<String, &'a ZilNode>,
    pub buzzi: Vec<&'a ZilNode>,
    pub synonyms: HashMap<String, &'a ZilNode>,
    pub syntax: Vec<&'a ZilNode>,
}

impl<'a> CrossRef<'a> {
    pub fn new(tree: &Tree) -> CrossRef {
        CrossRef {
            tree,
            rooms: HashMap::new(),
            objects: HashMap::new(),
            directions: None,
            routines: HashMap::new(),
            globals: HashMap::new(),
            gdecls: HashMap::new(),
            constants: HashMap::new(),
            buzzi: Vec::new(),
            synonyms: HashMap::new(),
            syntax: Vec::new(),
        }
    }

    pub fn populate(&mut self) {
        self.find_stuff();
    }

    fn find_stuff(&mut self) {
        let root = self.tree.get_root();
        self.find_stuff_recursive(root)
    }

    fn find_stuff_recursive(&mut self, node: &'a ZilNode) {
        for n in node.children.iter() {
            if n.node_type == ZilNodeType::Cluster {
                match get_nth_child_name(0, n) {
                    Some(name) => {
                        self.handle_named_cluster(n, name);
                    }
                    None => (),
                }
            }

            self.find_stuff_recursive(n);
        }
    }

    fn handle_named_cluster(&mut self, root: &'a ZilNode, name: String) {
        match name.as_str() {
            "ROOM" => match get_nth_child_name(1, root) {
                Some(name) => {
                    if self.rooms.contains_key(&name) {
                        panic!("Duplicate room name {}", name);
                    }
                    self.rooms.insert(name, root);
                }
                None => panic!("Room has no name"),
            },
            "OBJECT" => match get_nth_child_name(1, root) {
                Some(name) => {
                    if self.objects.contains_key(&name) {
                        panic!("Duplicate object name {}", name);
                    }
                    self.objects.insert(name, root);
                }
                None => panic!("Object has no name"),
            },
            "DIRECTIONS" => {
                self.directions = Some(root);
            }
            "ROUTINE" => match get_nth_child_name(1, root) {
                Some(name) => {
                    if self.routines.contains_key(&name) {
                        panic!("Duplicate routine name {}", name);
                    }
                    self.routines.insert(name, root);
                }
                None => panic!("Routine has no name"),
            },
            "GLOBAL" => match get_nth_child_name(1, root) {
                Some(name) => {
                    if self.globals.contains_key(&name) {
                        panic!("Duplicate global name {}", name);
                    }
                    self.globals.insert(name, root);
                }
                None => panic!("Global has no name"),
            },
            "GDECL" => match get_nth_child_name(1, root) {
                Some(name) => {
                    if self.gdecls.contains_key(&name) {
                        panic!("Duplicate gdecl name {}", name);
                    }
                    self.gdecls.insert(name, root);
                }
                None => panic!("Gdecl has no name"),
            },
            "CONSTANT" => match get_nth_child_name(1, root) {
                Some(name) => {
                    if self.constants.contains_key(&name) {
                        panic!("Duplicate constant name {}", name);
                    }
                    self.constants.insert(name, root);
                }
                None => panic!("Constant has no name"),
            },
            "BUZZ" => {
                self.buzzi.push(root);
            },
            "SYNONYM" => match get_nth_child_name(1, root) {
                Some(name) => {
                    if self.synonyms.contains_key(&name) {
                        panic!("Duplicate synonym name {}", name);
                    }
                    self.synonyms.insert(name, root);
                }
                None => panic!("Synonym has no name"),
            },
            "SYNTAX" => {
                self.syntax.push(root)
            }
            _ => (),
        }
    }
}
