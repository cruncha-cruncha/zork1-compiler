use std::fs::File;

use crate::zil::ast::{Node, NodeType};
use crate::js::handlers::generic_tokens::*;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

pub struct Divide {}

impl HandleJS for Divide {
    fn validate (root: &Node) -> Result<(), HandlerErr> {
        if root.children.len() != 3 {
            return Err(HandlerErr::origin(format!("Invalid Divide: {}", root)));
        }
        Ok(())
    }
  
    fn print(root: &Node, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;
      
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
        wrap!(writer.w(format!("{}(", spacer)));

        match root.children[1].kind() {
            NodeType::Routine => wrap!(R::print(&root.children[1], 0, &mut writer)),
            NodeType::Word => wrap!(W::print(&root.children[1], 0, &mut writer)),
            _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown NodeType in numerator of Divide"))),
        };

        wrap!(writer.w(" / "));

        match root.children[2].kind() {
            NodeType::Routine => wrap!(R::print(&root.children[2], 0, &mut writer)),
            NodeType::Word => wrap!(W::print(&root.children[2], 0, &mut writer)),
            _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown NodeType in denominator of Divide"))),
        };

        wrap!(writer.w(")"));

        Ok(())
    }
}