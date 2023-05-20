use std::collections::HashSet;

use crate::zil::node::{ZilNode, ZilNodeType};

use super::{helpers::get_bunch_name, top_level::Phodex};

pub struct GdeclPhodex<'a> {
    basis: Vec<&'a ZilNode>,
    pub globals: HashSet<String>,
}

impl<'a> GdeclPhodex<'a> {
    pub fn new() -> GdeclPhodex<'a> {
        GdeclPhodex {
            basis: Vec::new(),
            globals: HashSet::new(),
        }
    }
}

impl<'a> IntoIterator for GdeclPhodex<'a> {
    type Item = &'a ZilNode;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.basis.clone().into_iter()
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
                return Err(String::from(
                    "Gdecl node does not have exactly three children",
                ));
            }

            if n.children[1].node_type != ZilNodeType::Group {
                return Err(String::from(
                    "Gdecl node does not have a Group as its second child",
                ));
            }

            for c in n.children[1].children.iter() {
                match get_bunch_name(c) {
                    Some(name) => {
                        self.globals.insert(name);
                    }
                    None => panic!("Gdecl node has non-word child"),
                }
            }
        }

        Ok(())
    }
}
