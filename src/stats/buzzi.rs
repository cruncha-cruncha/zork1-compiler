use std::collections::HashSet;

use crate::zil::{node::ZilNode, file_table::format_file_location};

use super::{helpers::get_bunch_name, top_level::Phodex};

pub struct BuzzPhodex<'a> {
    basis: Vec<&'a ZilNode>,
    pub all: HashSet<String>,
}

impl<'a> BuzzPhodex<'a> {
    pub fn new() -> BuzzPhodex<'a> {
        BuzzPhodex {
            basis: Vec::new(),
            all: HashSet::new(),
        }
    }
}

impl<'a> IntoIterator for BuzzPhodex<'a> {
    type Item = &'a ZilNode;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.basis.clone().into_iter()
    }
}

impl<'a> Phodex<'a> for BuzzPhodex<'a> {
    fn get_name(&self) -> String {
        String::from("buzzi")
    }

    fn add_node(&mut self, node: &'a ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> Result<(), String> {
        for n in self.basis.iter() {
            for c in n.children.iter().skip(1) {
                match get_bunch_name(c) {
                    Some(name) => {
                        if !self.all.insert(name) {
                            panic!("Buzz node has duplicate child word {}", get_bunch_name(c).unwrap());
                        }
                    }
                    None => panic!("Buzz node has non-word child"),
                }
            }
        }

        Ok(())
    }
}
