use crate::zil::{
    ast::Tree,
    node::{ZilNode, ZilNodeType},
};

use super::{
    constants::ConstantCodex, directions::DirectionCodex, gdecls::GdeclPhodex,
    globals::GlobalCodex, helpers::get_nth_child_name, objects::ObjectCodex, rooms::RoomCodex, buzzi::BuzzPhodex, routines::RoutineCodex, synonyms::SynonymCodex, syntax::SyntaxPhodex, conds::CondPhodex,
};

pub trait Codex<'a>: IntoIterator<Item = String> {
    fn get_name(&self) -> String;
    fn add_node(&mut self, node: &'a ZilNode);
    fn crunch(&mut self) -> Result<(), String>;
    fn lookup(&self, word: &str) -> Option<&ZilNode>;
}

pub trait Phodex<'a>: IntoIterator<Item = &'a ZilNode> {
    fn get_name(&self) -> String;
    fn add_node(&mut self, node: &'a ZilNode);
    fn crunch(&mut self) -> Result<(), String>;
}

pub struct CrossRef<'a> {
    tree: &'a Tree,
    pub globals: GlobalCodex<'a>,
    pub gdecls: GdeclPhodex<'a>,
    pub constants: ConstantCodex<'a>,
    pub directions: DirectionCodex<'a>,
    pub rooms: RoomCodex<'a>,
    pub objects: ObjectCodex<'a>,
    pub buzzi: BuzzPhodex<'a>,
    pub routines: RoutineCodex<'a>,
    pub synonyms: SynonymCodex<'a>,
    pub syntax: SyntaxPhodex<'a>,
    pub conds: CondPhodex<'a>,
    pub leftovers: Vec<&'a ZilNode>,
}

impl<'a> CrossRef<'a> {
    pub fn new(tree: &Tree) -> CrossRef {
        CrossRef {
            tree,
            globals: GlobalCodex::new(),
            gdecls: GdeclPhodex::new(),
            constants: ConstantCodex::new(),
            directions: DirectionCodex::new(),
            rooms: RoomCodex::new(),
            objects: ObjectCodex::new(),
            routines: RoutineCodex::new(),
            buzzi: BuzzPhodex::new(),
            synonyms: SynonymCodex::new(),
            syntax: SyntaxPhodex::new(),
            conds: CondPhodex::new(),
            leftovers: Vec::new(),
        }
    }

    pub fn find_stuff(&mut self) {
        let root = self.tree.get_root();

        for n in root.children.iter() {
            if n.node_type == ZilNodeType::Cluster {
                match get_nth_child_name(0, n) {
                    Some(name) => {
                        self.handle_named_cluster(n, name);
                    }
                    None => self.leftovers.push(n),
                }
            } else {
                self.leftovers.push(n);
            }
        }

        if self.leftovers.len() > 0 {
            panic!("Leftover nodes after initial pass");
        }
    }

    fn handle_named_cluster(&mut self, root: &'a ZilNode, name: String) {
        match name.as_str() {
            "ROOM" => self.rooms.add_node(root),
            "OBJECT" => self.objects.add_node(root),
            "DIRECTIONS" => self.directions.add_node(root),
            "ROUTINE" => self.routines.add_node(root),
            "GLOBAL" => self.globals.add_node(root),
            "GDECL" => self.gdecls.add_node(root),
            "CONSTANT" => self.constants.add_node(root),
            "BUZZ" => self.buzzi.add_node(root),
            "SYNONYM" => self.synonyms.add_node(root),
            "SYNTAX" => self.syntax.add_node(root),
            "COND" => self.conds.add_node(root),
            _ => self.leftovers.push(root),
        }
    }
}
