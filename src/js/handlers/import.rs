use std::fs::File;

use crate::zil::ast::{Node, NodeType};
use crate::js::handlers::generic_tokens::*;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

pub struct Import {}

impl HandleJS for Import {
    fn validate (root: &Node) -> Result<(), HandlerErr> {
        if root.children.len() < 2 {
            return Err(HandlerErr::origin(format!("Invalid Import: {}", root)));
        }
        Ok(())
    }
  
    fn print(root: &Node, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;
      
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
        wrap!(writer.w(format!("{}import ", spacer)));

        match root.children[1].kind() {
          NodeType::Text => wrap!(T::print(&root.children[1], 0, &mut writer)),
          _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown NodeType in Import")))
        };
      
        wrap!(writer.w(";\n"));
      
        Ok(())
    }
}