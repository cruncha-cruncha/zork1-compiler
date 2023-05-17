use crate::zil::node::{TokenBunchType, ZilNode, ZilNodeType};

pub struct GdeclPhodex<'a> {
    basis: Vec<&'a ZilNode>,
}

impl<'a> IntoIterator for GdeclPhodex<'a> {
    type Item = &'a ZilNode;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.basis.into_iter()
    }
}

impl<'a> GdeclPhodex<'a> {
    pub fn new() -> GdeclPhodex<'a> {
        GdeclPhodex {
            basis: Vec::new(),
        }
    }

    pub fn get_name(&self) -> String {
        String::from("gdecls")
    }

    pub fn add_node(&mut self, node: &'a ZilNode) {
        self.basis.push(node);
    }

    pub fn crunch(&mut self) -> Result<(), String> {
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
                if c.node_type != ZilNodeType::TokenBunch(TokenBunchType::Word) {
                    return Err(String::from("Gdecl node has non-word child in group"));
                }
            }
        }

        Ok(())
    }
}
