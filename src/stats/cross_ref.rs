use std::sync::mpsc;

use crate::zil::{
    ast::Tree,
    node::{ZilNode, ZilNodeType}, file_table::format_file_location,
};

use super::{
    helpers::get_nth_child_name, top_level::buzzi::BuzzPhodex, top_level::constants::ConstantCodex,
    top_level::directions::DirectionCodex, top_level::gdecls::GdeclPhodex,
    top_level::globals::GlobalCodex, top_level::objects::ObjectCodex, top_level::rooms::RoomCodex,
    top_level::routines::RoutineCodex, top_level::synonyms::SynonymCodex,
    top_level::syntax::SyntaxPhodex, weaver::Sigourney,
};

pub trait Codex<'a> {
    fn get_name(&self) -> String;
    fn add_node(&mut self, node: &'a ZilNode);
    fn crunch(&mut self) -> Result<(), String>;
    fn lookup(&self, word: &str) -> Option<&ZilNode>;
    fn into_iter(&self) -> std::vec::IntoIter<String>;
}

pub trait Phodex<'a> {
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
            let lim = 10;

            for n in self.leftovers.iter().take(lim) {
                println!("{}", format_file_location(n));
            }

            if self.leftovers.len() > lim {
                println!("...");
            }

            panic!("Leftover nodes after initial pass");
        }
    }

    pub fn crunch<'b>(&mut self, thread_pool: &mut Sigourney) -> Result<(), String> {
        // crunch all sub info
        let mut receivers: Vec<mpsc::Receiver<Result<(), String>>> = Vec::with_capacity(10);
        receivers.push(thread_pool.run_fn(|| self.globals.crunch()));
        receivers.push(thread_pool.run_fn(|| self.gdecls.crunch()));
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
        receivers.push(thread_pool.run_fn(|| self.gdecls.validate_against_globals(&self.globals)));
        receivers.push(thread_pool.run_fn(|| self.syntax.validate_actions(&self.routines)));
        receivers.push(thread_pool.run_fn(|| self.rooms.validate_direction_names(&self.directions)));
        receivers.push(thread_pool.run_fn(|| self.rooms.validate_objects(&self.objects)));
        receivers.push(thread_pool.run_fn(|| self.rooms.validate_globals(&self.globals)));
        receivers.push(thread_pool.run_fn(|| self.rooms.validate_routines(&self.routines)));
        receivers.push(thread_pool.run_fn(|| self.rooms.validate_rooms()));
        receivers.push(thread_pool.run_fn(|| self.objects.validate_routines(&self.routines)));
        receivers.push(thread_pool.run_fn(|| self.objects.validate_room_or_object(&self.rooms)));
        receivers.push(thread_pool.run_fn(|| self.objects.validate_objects()));

        // wait for all threads to finish
        for receiver in receivers.iter_mut() {
            match receiver.recv().unwrap() {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        Ok(())
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
            _ => self.leftovers.push(root),
        }
    }
}
