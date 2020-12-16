use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::zil::ast::*;

use crate::js::handle::generic_tokens::*;

pub fn handle_divide(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
  if root.children.len() != 3 {
      return Err(());
  }

  let spacer = (0..indent).map(|_| "  ").collect::<String>();
  writer.write(format!("{}(", spacer).as_bytes());

  match root.children[1].kind() {
      NodeType::Routine => handle_r(&root.children[1], 0, &mut writer),
      NodeType::Word => handle_w(&root.children[1], 0, &mut writer),
      _ => Err(()),
  }?;

  writer.write(b" / ");

  match root.children[2].kind() {
      NodeType::Routine => handle_r(&root.children[2], 0, &mut writer),
      NodeType::Word => handle_w(&root.children[2], 0, &mut writer),
      _ => Err(()),
  }?;

  writer.write(b")");

  Ok(())
}