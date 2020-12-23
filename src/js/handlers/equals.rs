use std::fs::File;

use crate::zil::ast::{Node, NodeType};
use crate::js::handlers::generic_tokens::*;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

pub struct Equals {}

impl HandleJS for Equals {
    fn validate (root: &Node) -> Result<(), HandlerErr> {
        if !root.is_routine() ||
           root.children.len() < 3 ||
           !root.children[0].is_word() {
            return Err(HandlerErr::origin(format!("Invalid Equals: {}", root)));
        }
        Ok(())
    }
  
    fn print(root: &Node, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;
      
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
        wrap!(writer.w(format!("{}", spacer)));

        for i in 2..root.children.len() {
            wrap!(W::print(&root.children[1], 0, &mut writer));
            wrap!(writer.w(" == "));
            match root.children[i].kind() {
                NodeType::Routine => wrap!(R::print(&root.children[i], 0, &mut writer)),
                NodeType::Text => wrap!(T::print(&root.children[i], 0, &mut writer)),
                NodeType::Word => wrap!(W::print(&root.children[i], 0, &mut writer)),
                _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown NodeType in Equals"))),
            };
            if i+1 < root.children.len() {
                wrap!(writer.w(" || "));
            }
        }
      
        Ok(())
    }
}