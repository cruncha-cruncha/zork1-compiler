use crate::zil::contracts::*;
use crate::inter::contracts::*;
use crate::inter;

pub fn clone_zil_tree(root: &ZilNode) -> Result<InterNode, InterErr> {
  let root = inter::contracts::InterNode::clone_zilnode(&root)?;
  inter::validation::validate(&root)?;
  Ok(root)
}