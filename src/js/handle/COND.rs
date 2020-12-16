use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::zil::ast::*;

use crate::js::handle::generic_tokens::*;

pub fn handle_COND(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
  if root.children.len() < 2 ||
     !root.children[1].is_grouping() || 
     root.children[1].children.len() < 2 {
      return Err(());
  }

  let spacer = (0..indent).map(|_| "  ").collect::<String>();
  writer.write(format!("{}if ", spacer).as_bytes());

  for g in 1..root.children.len() {
      writer.write(b"(");
      match root.children[g].children[0].kind() {
          NodeType::Routine => handle_r(&root.children[g].children[0], 0, &mut writer),       
          NodeType::Word => handle_w(&root.children[g].children[0], 0, &mut writer),
          _ => Err(()),
      }?;
      writer.write(b") {\n");
  
      for i in 1..root.children[g].children.len() {
          match root.children[g].children[i].kind() {
              NodeType::Routine => handle_r(&root.children[g].children[i], indent+1, &mut writer),
              NodeType::Word => handle_w(&root.children[g].children[i], indent+1, &mut writer),
              _ => Err(()),
          }?;
          writer.write(b"\n");
      }
      if g+1 < root.children.len() {
          writer.write(format!("{}}} else if ", spacer).as_bytes());
      }
  }

  writer.write(format!("{}}}\n", spacer).as_bytes());
  
  Ok(())
}