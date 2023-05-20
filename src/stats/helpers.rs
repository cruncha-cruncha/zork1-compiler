use crate::zil::{
    ast::Tree,
    node::{ZilNode, ZilNodeType, TokenBunchType},
};

#[allow(dead_code)]
pub struct Helpers<'a> {
    tree: &'a Tree,
    pub cluster_names: Vec<String>,
    pub top_level_cluster_names: Vec<String>,
    pub top_level_node_types: Vec<ZilNodeType>,
}

#[allow(dead_code)]
impl<'a> Helpers<'a> {
    pub fn new(tree: &Tree) -> Helpers {
        Helpers {
            tree,
            cluster_names: Vec::new(),
            top_level_cluster_names: Vec::new(),
            top_level_node_types: Vec::new(),
        }
    }

    pub fn populate(&mut self) {
        self.populate_cluster_names();
        self.populate_top_level();
    }

    fn populate_cluster_names(&mut self) {
        let root = self.tree.get_root();
        self.populate_cluster_names_recursive(root)
    }

    fn populate_cluster_names_recursive(&mut self, node: &'a ZilNode) {
        for n in node.children.iter() {
            if n.node_type == ZilNodeType::Cluster {
                match get_nth_child_name(0, n) {
                    Some(name) => {
                        if !self.cluster_names.contains(&name) {
                            self.cluster_names.push(name);
                        }
                    }
                    None => (),
                }
            }

            self.populate_cluster_names_recursive(n);
        }
    }

    fn populate_top_level(&mut self) {
        let root = self.tree.get_root();
        for n in root.children.iter() {
            if !self.top_level_node_types.contains(&n.node_type) {
                self.top_level_node_types.push(n.node_type);
            }

            if n.node_type == ZilNodeType::Cluster {
                match get_nth_child_name(0, n) {
                    Some(name) => {
                        if !self.top_level_cluster_names.contains(&name) {
                            self.top_level_cluster_names.push(name);
                        }
                    }
                    None => (),
                }
            }
        }
    }
}

pub fn get_nth_child_name(n: usize, node: &ZilNode) -> Option<String> {
    if node.children.len() <= n {
        return None;
    }

    get_bunch_name(&node.children[n])
}

pub fn get_bunch_name(node: &ZilNode) -> Option<String> {
    match node.node_type {
        ZilNodeType::TokenBunch(TokenBunchType::Word) => (),
        _ => return None,
    }

    let mut str_buf = String::new();
    for cc in node.children.iter() {
        str_buf.push_str(&cc.token_val());
    }

    Some(str_buf)
}