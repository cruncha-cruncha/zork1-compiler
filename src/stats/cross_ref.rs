use std::sync::mpsc;

use crate::zil::{
    ast::Tree,
    file_table::format_file_location,
    node::{ZilNode, ZilNodeType},
};

use super::{
    helpers::get_nth_child_as_word,
    top_level::{
        buzzi::BuzzStats, constants::ConstantStats, directions::DirectionStats,
        globals::GlobalStats, objects::ObjectStats, rooms::RoomStats, routines::RoutineStats,
        synonyms::SynonymStats, syntax::SyntaxStats,
    },
    weaver::Sigourney,
};

pub trait Populator<'a> {
    fn add_node(&mut self, node: &'a ZilNode);
    fn crunch(&mut self) -> Result<(), String>;
    fn validate(&self, cross_ref: &CrossRef) -> Result<(), String>;
}

pub trait Codex {
    fn lookup(&self, word: &str) -> Option<&ZilNode>;
}

pub struct CrossRef<'a> {
    tree: &'a Tree,
    pub globals: GlobalStats<'a>,
    pub constants: ConstantStats<'a>,
    pub directions: DirectionStats<'a>,
    pub rooms: RoomStats<'a>,
    pub objects: ObjectStats<'a>,
    pub buzzi: BuzzStats<'a>,
    pub routines: RoutineStats<'a>,
    pub synonyms: SynonymStats<'a>,
    pub syntax: SyntaxStats<'a>,
    pub leftovers: Vec<&'a ZilNode>,
}

impl<'a> CrossRef<'a> {
    pub fn new(tree: &Tree) -> CrossRef {
        CrossRef {
            tree,
            globals: GlobalStats::new(),
            constants: ConstantStats::new(),
            directions: DirectionStats::new(),
            rooms: RoomStats::new(),
            objects: ObjectStats::new(),
            routines: RoutineStats::new(),
            buzzi: BuzzStats::new(),
            synonyms: SynonymStats::new(),
            syntax: SyntaxStats::new(),
            leftovers: Vec::new(),
        }
    }

    pub fn add_nodes(&mut self) {
        let root = self.tree.get_root();

        for n in root.children.iter() {
            if n.node_type == ZilNodeType::Cluster {
                match get_nth_child_as_word(0, n) {
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
            let lim = 10;

            for n in self.leftovers.iter().take(lim) {
                println!("{}", format_file_location(n));
            }

            if self.leftovers.len() > lim {
                println!("...");
            }

            panic!("Leftover top-level nodes after initial pass");
        }
    }

    pub fn crunch_top_level(&mut self, thread_pool: &mut Sigourney) -> Result<(), String> {
        // crunch all sub info
        let mut receivers: Vec<mpsc::Receiver<Result<(), String>>> = Vec::with_capacity(10);
        receivers.push(thread_pool.run_fn(|| self.globals.crunch()));
        receivers.push(thread_pool.run_fn(|| self.constants.crunch()));
        receivers.push(thread_pool.run_fn(|| self.directions.crunch()));
        receivers.push(thread_pool.run_fn(|| self.rooms.crunch()));
        receivers.push(thread_pool.run_fn(|| self.objects.crunch()));
        receivers.push(thread_pool.run_fn(|| self.buzzi.crunch()));
        receivers.push(thread_pool.run_fn(|| self.routines.crunch()));
        receivers.push(thread_pool.run_fn(|| self.synonyms.crunch()));
        receivers.push(thread_pool.run_fn(|| self.syntax.crunch()));

        // wait for all threads to finish
        for receiver in receivers.iter_mut() {
            match receiver.recv().unwrap() {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        receivers.clear();

        // validate cross references
        receivers.push(thread_pool.run_fn(|| self.globals.validate(&self)));
        receivers.push(thread_pool.run_fn(|| self.constants.validate(&self)));
        receivers.push(thread_pool.run_fn(|| self.directions.validate(&self)));
        receivers.push(thread_pool.run_fn(|| self.rooms.validate(&self)));
        receivers.push(thread_pool.run_fn(|| self.objects.validate(&self)));
        receivers.push(thread_pool.run_fn(|| self.buzzi.validate(&self)));
        receivers.push(thread_pool.run_fn(|| self.routines.validate(&self)));
        receivers.push(thread_pool.run_fn(|| self.synonyms.validate(&self)));
        receivers.push(thread_pool.run_fn(|| self.syntax.validate(&self)));

        // wait for all threads to finish
        for receiver in receivers.iter_mut() {
            match receiver.recv().unwrap() {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    pub fn validate_routines(&self) -> Result<(), String> {
        let v = super::validate_recursive::Validator::new(self);

        self.routines.validate_recursive(&v)
    }

    fn handle_named_cluster(&mut self, root: &'a ZilNode, name: String) {
        match name.as_str() {
            "ROOM" => self.rooms.add_node(root),
            "OBJECT" => self.objects.add_node(root),
            "DIRECTIONS" => self.directions.add_node(root),
            "ROUTINE" => self.routines.add_node(root),
            "GLOBAL" => self.globals.add_node(root),
            "CONSTANT" => self.constants.add_node(root),
            "BUZZ" => self.buzzi.add_node(root),
            "SYNONYM" => self.synonyms.add_node(root),
            "SYNTAX" => self.syntax.add_node(root),
            _ => self.leftovers.push(root),
        }
    }
}
