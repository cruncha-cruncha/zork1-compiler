use std::fs::File;

use crate::zil::ast::{Node, NodeType};
use crate::js::handlers::generic_tokens::*;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

pub struct Multiply {}

impl HandleJS for Multiply {
    fn validate (root: &Node) -> Result<(), HandlerErr> {
        if root.children.len() < 3 {
            return Err(HandlerErr::origin(format!("Invalid Multiply: {}", root)));
        }
        Ok(())
    }
  
    fn print(root: &Node, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;
      
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
        wrap!(writer.w(format!("{}(", spacer)));

        for i in 1..root.children.len() {
            match root.children[i].kind() {
                NodeType::Routine => wrap!(R::print(&root.children[i], 0, &mut writer)),
                NodeType::Word => wrap!(W::print(&root.children[i], 0, &mut writer)),
                _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown NodeType in Multiply"))),
            };
            if i+1 < root.children.len() {
                wrap!(writer.w(" * "));
            }
        }

        wrap!(writer.w(")"));

        Ok(())
    }
}