use crate::zil::{
    ast::Tree,
    node::{ZilNode, ZilNodeType},
};

use super::{
    directions::DirectionCodex,
    helpers::get_nth_child_name, globals::GlobalCodex, gdecls::GdeclPhodex, rooms::RoomCodex,
};

pub trait Codex<'a>: IntoIterator<Item = String> {
    fn get_name(&self) -> String;
    fn add_node(&mut self, node: &'a ZilNode);
    fn crunch(&mut self) -> Result<(), String>;
    fn lookup(&self, word: &str) -> Option<&ZilNode>;
}

pub struct CrossRef<'a> {
    tree: &'a Tree,
    pub globals: GlobalCodex<'a>,
    pub gdecls: GdeclPhodex<'a>,
    // pub constants: HashMap<String, &'a ZilNode>,
    pub directions: DirectionCodex<'a>,
    pub rooms: RoomCodex<'a>,
    // pub objects: HashMap<String, &'a ZilNode>,
    // pub buzzi: Vec<&'a ZilNode>,
    // pub routines: HashMap<String, &'a ZilNode>,
    // pub synonyms: HashMap<String, &'a ZilNode>,
    // pub syntax: Vec<&'a ZilNode>,
}

impl<'a> CrossRef<'a> {
    pub fn new(tree: &Tree) -> CrossRef {
        CrossRef {
            tree,
            globals: GlobalCodex::new(),
            gdecls: GdeclPhodex::new(),
            // constants: HashMap::new(),
            directions: DirectionCodex::new(),
            rooms: RoomCodex::new(),
            // objects: HashMap::new(),
            // routines: HashMap::new(),
            // buzzi: Vec::new(),
            // synonyms: HashMap::new(),
            // syntax: Vec::new(),
        }
    }

    pub fn find_stuff(&mut self) {
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
            "ROOM" => self.rooms.add_node(root),
            // "OBJECT" => match get_nth_child_name(1, root) {
            //     Some(name) => {
            //         if self.objects.contains_key(&name) {
            //             panic!("Duplicate object name {}", name);
            //         }
            //         self.objects.insert(name, root);
            //     }
            //     None => panic!("Object has no name"),
            // },
            "DIRECTIONS" => self.directions.add_node(root),
            // "ROUTINE" => match get_nth_child_name(1, root) {
            //     Some(name) => {
            //         if self.routines.contains_key(&name) {
            //             panic!("Duplicate routine name {}", name);
            //         }
            //         self.routines.insert(name, root);
            //     }
            //     None => panic!("Routine has no name"),
            // },
            "GLOBAL" => self.globals.add_node(root),
            "GDECL" => self.gdecls.add_node(root),
            // "CONSTANT" => match get_nth_child_name(1, root) {
            //     Some(name) => {
            //         if self.constants.contains_key(&name) {
            //             panic!("Duplicate constant name {}", name);
            //         }
            //         self.constants.insert(name, root);
            //     }
            //     None => panic!("Constant has no name"),
            // },
            // "BUZZ" => {
            //     self.buzzi.push(root);
            // }
            // "SYNONYM" => match get_nth_child_name(1, root) {
            //     Some(name) => {
            //         if self.synonyms.contains_key(&name) {
            //             panic!("Duplicate synonym name {}", name);
            //         }
            //         self.synonyms.insert(name, root);
            //     }
            //     None => panic!("Synonym has no name"),
            // },
            // "SYNTAX" => self.syntax.push(root),
            _ => (),
        }
    }
}
