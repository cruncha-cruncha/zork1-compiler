use std::sync::mpsc;

use crate::zil::{
    ast::Tree,
    file_table::format_file_location,
    node::{ZilNode, ZilNodeType},
};

use super::{
    helpers::get_nth_child_as_word,
    meta_handler::MetaHandler,
    top_level::{
        buzzi::BuzzStats, constants::ConstantStats, directions::DirectionStats,
        globals::GlobalStats, objects::ObjectStats, rooms::RoomStats, routines::RoutineStats,
        synonyms::SynonymStats, syntax::SyntaxStats,
    },
    weaver::Sigourney,
};

pub trait Populator {
    fn add_node(&mut self, node: ZilNode);
    fn crunch(&mut self) -> Result<(), String>;
    fn validate(&self, cross_ref: &CrossRef) -> Result<(), String>;
}

pub trait Codex {
    fn lookup(&self, word: &str) -> Option<&ZilNode>;
}

pub struct CrossRef {
    tree: Option<Tree>,
    pub globals: GlobalStats,
    pub constants: ConstantStats,
    pub directions: DirectionStats,
    pub rooms: RoomStats,
    pub objects: ObjectStats,
    pub buzzi: BuzzStats,
    pub routines: RoutineStats,
    pub synonyms: SynonymStats,
    pub syntax: SyntaxStats,
    pub others: Vec<ZilNode>,
}

impl CrossRef {
    pub fn new(tree: Tree) -> CrossRef {
        CrossRef {
            tree: Some(tree),
            globals: GlobalStats::new(),
            constants: ConstantStats::new(),
            directions: DirectionStats::new(),
            rooms: RoomStats::new(),
            objects: ObjectStats::new(),
            routines: RoutineStats::new(),
            buzzi: BuzzStats::new(),
            synonyms: SynonymStats::new(),
            syntax: SyntaxStats::new(),
            others: Vec::new(),
        }
    }

    pub fn add_nodes(&mut self) {
        if self.tree.is_none() {
            panic!("CrossRef has no tree");
        }

        let root = self.tree.take().unwrap().root;

        for n in root.children.into_iter() {
            if n.node_type == ZilNodeType::Cluster {
                match get_nth_child_as_word(0, &n) {
                    Some(name) => {
                        self.handle_named_cluster(n, name);
                    }
                    None => self.others.push(n),
                }
            } else {
                self.others.push(n);
            }
        }

        if self.others.len() > 0 {
            let lim = 10;

            for n in self.others.iter().take(lim) {
                println!("unknown top-level node at: {}", format_file_location(n));
            }

            if self.others.len() > lim {
                println!("...");
            }
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

    pub fn validate_routines(&mut self) -> Result<(), String> {
        let meta_handler = MetaHandler::new();

        let replaced_something = self.routines.resolve_meta_code(&meta_handler)?;

        println!(
            "resolved meta code, replaced_something: {}",
            replaced_something
        );

        self.routines.remove_decls();

        let v = super::validate_recursive::Validator::new(self);

        self.routines.validate_recursive(&v)
    }

    fn handle_named_cluster(&mut self, root: ZilNode, name: String) {
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
            _ => self.others.push(root),
        }
    }
}
