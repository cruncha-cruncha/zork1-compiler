use std::collections::HashSet;

use crate::zil::node::{ZilNode};

use super::helpers::{get_bunch_name, Has};

pub struct DirectionCodex<'a> {
    basis: &'a ZilNode,
    pub options: HashSet<String>,
}

impl<'a> DirectionCodex<'a> {
    pub fn new(basis: &'a ZilNode) -> DirectionCodex<'a> {
        DirectionCodex {
            basis,
            options: HashSet::new(),
        }
    }

    pub fn populate(&mut self) {
        self.populate_options();
    }

    fn populate_options(&mut self) {
        if self.basis.children.len() <= 0 {
            panic!("No directions");
        }

        for node in self.basis.children.iter().skip(1) {
            self.options.insert(get_bunch_name(node));
        }
    }
}

impl<'a> Has<&str> for DirectionCodex<'a> {
    fn has(&self, name: &str) -> bool {
        self.options.contains(&name.to_string())
    }
}