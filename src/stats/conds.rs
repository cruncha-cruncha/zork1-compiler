use crate::zil::node::ZilNode;

use super::top_level::Phodex;

pub struct CondPhodex<'a> {
    basis: Vec<&'a ZilNode>,
}

impl<'a> CondPhodex<'a> {
    pub fn new() -> CondPhodex<'a> {
        CondPhodex { basis: Vec::new() }
    }
}

impl<'a> IntoIterator for CondPhodex<'a> {
    type Item = &'a ZilNode;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.basis.clone().into_iter()
    }
}

impl<'a> Phodex<'a> for CondPhodex<'a> {
    fn get_name(&self) -> String {
        String::from("conds")
    }

    fn add_node(&mut self, node: &'a ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> Result<(), String> {
        // TODO
        Ok(())
    }
}
