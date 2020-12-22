use std::fs::File;

use crate::zil::ast::{Node, NodeType};
use crate::js::handlers::generic_tokens::*;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

pub struct SET {}

impl HandleJS for SET {
    fn validate (root: &Node) -> Result<(), HandlerErr> {
        if root.children.len() != 3 {
            return Err(HandlerErr::origin(format!("Invalid SET: {}", root)));
        }
        Ok(())
    }
  
    fn print(root: &Node, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;
      
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
        wrap!(writer.w(format!("{}(", spacer)));

        match root.children[1].kind() {
            NodeType::Word => wrap!(W::print(&root.children[1], 0, &mut writer)),
            _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown NodeType in name in SET"))),
        };

        wrap!(writer.w(" = "));

        match root.children[2].kind() {
            NodeType::Routine => wrap!(R::print(&root.children[2], 0, &mut writer)),
            NodeType::Text => wrap!(T::print(&root.children[2], 0, &mut writer)),
            NodeType::Word => wrap!(W::print(&root.children[2], 0, &mut writer)),
            _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown NodeType in value in SET"))),
        };

        wrap!(writer.w(")"));

        Ok(())
    }
}