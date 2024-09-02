use crate::zil::node::ZilNodeType;
use crate::zil::{file_table::format_file_location, node::ZilNode};

pub struct MetaHandler {}

struct Replacement {
    index: usize,
    node: Option<ZilNode>,
}

impl MetaHandler {
    pub fn new() -> MetaHandler {
        MetaHandler {}
    }

    pub fn replace_meta_recursively(&self, n: &mut ZilNode) -> Result<bool, String> {
        let mut replaced_something = false;
        let mut replacements: Vec<Replacement> = Vec::new();

        for (i, c) in n.children.iter_mut().enumerate() {
            if c.node_type == ZilNodeType::Cluster && c.token_val().unwrap_or_default() == "%" {
                let replacement = self.find_replacement(c);

                if replacement.is_none() {
                    return Err(format!(
                        "Routine node has %< without ' child\n{}",
                        format_file_location(&c)
                    ));
                }

                replacements.push(Replacement {
                    index: i,
                    node: replacement,
                });

                replaced_something = true;
            } else if c.node_type == ZilNodeType::Cluster || c.node_type == ZilNodeType::Group {
                let status = self.replace_meta_recursively(c);

                if status.is_err() {
                    return status;
                } else if status.unwrap_or_default() {
                    replaced_something = true;
                }
            }
        }

        for r in replacements.iter_mut() {
            n.children[r.index] = r.node.take().unwrap();
        }

        Ok(replaced_something)
    }

    fn find_replacement(&self, n: &ZilNode) -> Option<ZilNode> {
        for c in n.children.iter() {
            if c.node_type == ZilNodeType::Cluster || c.node_type == ZilNodeType::Group {
                let has_tick = c.token_val().unwrap_or_default() == "'";

                if has_tick {
                    return Some(c.clone());
                }

                let deeper_child = self.find_replacement(c);

                if deeper_child.is_some() {
                    return deeper_child;
                }
            }
        }

        None
    }
}
