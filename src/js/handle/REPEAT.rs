use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::zil::ast::*;

use crate::js::handle::generic_tokens::*;

pub fn handle_REPEAT(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
  if root.children.len() < 3 {
      return Err(());
  }

  let spacer = (0..indent).map(|_| "  ").collect::<String>();
  writer.write(format!("{}(() => {{\n", spacer).as_bytes());
  writer.write(format!("{}  while (true) {{\n", spacer).as_bytes());

  for i in 2..root.children.len() {
      match root.children[i].kind() {
          NodeType::Routine => handle_r(&root.children[i], indent+2, &mut writer),
          _ => Err(()),
      }?;
      writer.write(b"\n");
  }

  writer.write(format!("{}  }}\n", spacer).as_bytes());
  writer.write(format!("{}}})()\n", spacer).as_bytes());

  Ok(())
}