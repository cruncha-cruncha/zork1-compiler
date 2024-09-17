use std::sync::mpsc;

use crate::zil::{
    ast::Tree,
    file_table::format_file_location,
    node::{ZilNode, ZilNodeType},
};

use super::{
    helpers::{parse_token_as_word, ValidationResult},
    routine_tracker::Validator,
    top_level::{
        buzzi::BuzzStats, directions::DirectionStats, globals::GlobalStats, objects::ObjectStats,
        player::PlayerStats, rooms::RoomStats, routines::RoutineStats, synonyms::SynonymStats,
        syntax::SyntaxStats,
    },
    weaver::Sigourney,
};

pub trait Populator {
    fn add_node(&mut self, node: ZilNode);
    fn crunch(&mut self) -> ValidationResult<()>;
    fn validate(&self, cross_ref: &CrossRef) -> ValidationResult<()>;
}

pub trait Codex<T> {
    fn lookup(&self, key: &str) -> Option<T>;
}

pub struct CrossRef {
    tree: Option<Tree>,
    pub player: PlayerStats,
    pub globals: GlobalStats,
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
            player: PlayerStats::new(),
            globals: GlobalStats::new(),
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

    pub fn name_is_illegal(name: &str) -> bool {
        match name {
            "CURRENT-ROOM" | "CMD" | "PRSO" | "PRSI" | "PLAYER" => true,
            _ => false,
        }
    }

    pub fn add_nodes(&mut self) {
        if self.tree.is_none() {
            panic!("CrossRef has no tree");
        }

        let root = self.tree.take().unwrap().root;

        for n in root.children.into_iter() {
            if n.node_type == ZilNodeType::Cluster {
                match parse_token_as_word(&n.children[0]) {
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

    pub fn crunch_top_level(&mut self, thread_pool: &mut Sigourney) -> ValidationResult<()> {
        let mut receivers: Vec<mpsc::Receiver<ValidationResult<()>>> = Vec::with_capacity(10);

        {
            // crunch all sub info
            receivers.push(thread_pool.run_fn(|| self.player.crunch()));
            receivers.push(thread_pool.run_fn(|| self.globals.crunch()));
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
        }

        {
            // validate cross references
            receivers.push(thread_pool.run_fn(|| self.player.validate(&self)));
            receivers.push(thread_pool.run_fn(|| self.globals.validate(&self)));
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

            receivers.clear();
        }

        self.player.nest_objects(self.objects.as_codex());
        self.rooms.nest_objects(self.objects.as_codex());
        self.objects.nest_objects();

        Ok(())
    }

    pub fn validate_routines_recursive(&self) -> Result<Validator, String> {
        let mut validator = Validator::new(self);

        for routine in self.routines.as_codex() {
            validator.validate_cluster(routine.node)?;
        }

        Ok(validator)
    }

    pub fn validate_unique_names(&self) -> Result<(), String> {
        let global_codex = self.globals.as_codex();
        let room_codex = self.rooms.as_codex();
        let object_codex = self.objects.as_codex();

        for global in global_codex {
            if room_codex.lookup(global.name).is_some() {
                return Err(format!("Global name is same as room name: {}", global.name));
            }

            if object_codex.lookup(global.name).is_some() {
                return Err(format!(
                    "Global name is same as object name: {}",
                    global.name
                ));
            }
        }

        for room in room_codex {
            if object_codex.lookup(room.name).is_some() {
                return Err(format!("Room name is same as object name: {}", room.name));
            }
        }

        Ok(())
    }

    fn handle_named_cluster(&mut self, root: ZilNode, name: String) {
        match name.as_str() {
            "PLAYER" => self.player.add_node(root),
            "ROOM" => self.rooms.add_node(root),
            "OBJECT" => self.objects.add_node(root),
            "DIRECTIONS" => self.directions.add_node(root),
            "ROUTINE" => self.routines.add_node(root),
            "GLOBAL" => self.globals.add_node(root),
            "BUZZ" => self.buzzi.add_node(root),
            "SYNONYM" => self.synonyms.add_node(root),
            "SYNTAX" => self.syntax.add_node(root),
            _ => self.others.push(root),
        }
    }
}
