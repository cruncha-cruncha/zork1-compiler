use std::fs::File;

use crate::zil::ast::{Node, NodeType};
use crate::js::handlers::generic_tokens::*;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

pub struct REPEAT {}

impl HandleJS for REPEAT {
    fn validate (root: &Node) -> Result<(), HandlerErr> {
        if root.children.len() < 3 {
            return Err(HandlerErr::origin(format!("Invalid REPEAT: {}", root)));
        }
        Ok(())
    }
  
    fn print(root: &Node, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;
      
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
        wrap!(writer.w(format!("{}(() => {{\n", spacer)));
        wrap!(writer.w(format!("{}  while (true) {{\n", spacer)));

        for i in 2..root.children.len() {
            match root.children[i].kind() {
                NodeType::Routine => wrap!(R::print(&root.children[i], indent+2, &mut writer)),
                _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown NodeType in REPEAT"))),
            };
            wrap!(writer.w("\n"));
        }

        wrap!(writer.w(format!("{}  }}\n", spacer)));
        wrap!(writer.w(format!("{}}})()\n", spacer)));

        Ok(())
    }
}