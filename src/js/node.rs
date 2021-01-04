use std::fs::File;
use std::io;

use crate::inter::node::*;
use crate::js::custom_buf_writer::*;

pub trait HandleJS {
  fn print (root: &JSNode, writer: &mut CustomBufWriter<File>) -> Result<(), io::Error>;
}

#[derive(Copy, Clone, PartialEq)]
pub enum JSNodeType {
  Unknown,
  Routine,
  EmptyRoutine,
  Grouping,
  Text,
  Word,
  Int
}

impl From<InterNodeType> for JSNodeType {
  fn from(kind: InterNodeType) -> Self {
    match kind {
      InterNodeType::Unknown => JSNodeType::Unknown,
      InterNodeType::Routine => JSNodeType::Routine,
      InterNodeType::EmptyRoutine => JSNodeType::EmptyRoutine,
      InterNodeType::Grouping => JSNodeType::Grouping,
      InterNodeType::Text => JSNodeType::Text,
      InterNodeType::Word => JSNodeType::Word,
      InterNodeType::Int => JSNodeType::Int
    }
  }
}

pub struct JSNode {
  pub kind: JSNodeType,
  pub value: String,
  pub children: Vec<JSNode>
}

impl JSNode {
  pub fn clone_internode(root: &InterNode) -> JSNode {
    let mut children = Vec::new();
    for i in 0..root.children.len() {
        children.push(Self::clone_internode(&root.children[i]))
    }
    JSNode {
      kind: JSNodeType::from(root.kind),
      value: root.value.clone(),
      children: children
    }
  }
}