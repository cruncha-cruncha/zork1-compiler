use std::collections::HashMap;

use crate::{
    stats::cross_ref::Codex,
    zil::{
        file_table::format_file_location,
        node::{ZilNode, ZilNodeType},
    },
};

use crate::stats::{cross_ref::Phodex, helpers::get_bunch_name};

pub struct GdeclPhodex<'a> {
    basis: Vec<&'a ZilNode>,
    pub globals: HashMap<String, Vec<&'a ZilNode>>,
}

impl<'a> GdeclPhodex<'a> {
    pub fn new() -> GdeclPhodex<'a> {
        GdeclPhodex {
            basis: Vec::new(),
            globals: HashMap::new(),
        }
    }

    pub fn validate_against_globals(&mut self, globals: &impl Codex<'a>) -> Result<(), String> {
        for g_name in self.globals.keys() {
            if globals.lookup(g_name).is_none() {
                return Err(format!("Global {} not found", g_name));
            }
        }

        Ok(())
    }
}

impl<'a> Phodex<'a> for GdeclPhodex<'a> {
    fn get_name(&self) -> String {
        String::from("gdecls")
    }

    fn add_node(&mut self, node: &'a ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> Result<(), String> {
        for n in self.basis.iter() {
            if n.children.len() != 3 {
                return Err(format!(
                    "Gdecl node does not have exactly three children\n{}",
                    format_file_location(n)
                ));
            }

            if n.children[1].node_type != ZilNodeType::Group {
                return Err(format!(
                    "Gdecl node does not have a Group as its second child\n{}",
                    format_file_location(n)
                ));
            }

            for c in n.children[1].children.iter() {
                match get_bunch_name(c) {
                    Some(name) => {
                        if self.globals.contains_key(&name) {
                            self.globals.get_mut(&name).unwrap().push(n);
                        } else {
                            self.globals.insert(name, vec![n]);
                        }
                    }
                    None => panic!(
                        "Gdecl node has non-word child in group\n{}",
                        format_file_location(n)
                    ),
                }
            }
        }

        Ok(())
    }
}
