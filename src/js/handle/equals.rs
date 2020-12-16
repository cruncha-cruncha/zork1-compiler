use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::zil::ast::*;

use crate::js::handle::generic_tokens::*;

pub fn handle_equals(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
  if root.children.len() < 3 {
      return Err(());
  }

  let spacer = (0..indent).map(|_| "  ").collect::<String>();
  writer.write(format!("{}", spacer).as_bytes());

  for i in 2..root.children.len() {
      handle_w(&root.children[1], 0, &mut writer)?;
      writer.write(b" == ");
      match root.children[i].kind() {
          NodeType::Routine => handle_r(&root.children[i], 0, &mut writer),
          NodeType::Text => handle_t(&root.children[i], 0, &mut writer),
          NodeType::Word => handle_w(&root.children[i], 0, &mut writer),
          _ => Err(()),
      }?;
      if i+1 < root.children.len() {
          writer.write(b" || ");
      }
  }

  Ok(())
}