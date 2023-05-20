use crate::zil::node::ZilNode;

use super::top_level::Phodex;

pub struct SyntaxPhodex<'a> {
    basis: Vec<&'a ZilNode>,
}

impl<'a> SyntaxPhodex<'a> {
    pub fn new() -> SyntaxPhodex<'a> {
        SyntaxPhodex { basis: Vec::new() }
    }
}

impl<'a> IntoIterator for SyntaxPhodex<'a> {
    type Item = &'a ZilNode;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.basis.clone().into_iter()
    }
}

impl<'a> Phodex<'a> for SyntaxPhodex<'a> {
    fn get_name(&self) -> String {
        String::from("syntax")
    }

    fn add_node(&mut self, node: &'a ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> Result<(), String> {
        // TODO
        Ok(())
    }
}
