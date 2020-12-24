use std::fs::File;

use crate::zil::contracts::*;
use crate::js::handlers::generic_tokens::*;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

pub struct NOT {}

impl HandleJS for NOT {
    fn validate (root: &ZilNode) -> Result<(), HandlerErr> {
        if !root.is_routine() ||
           root.children.len() != 2 ||
           !root.children[0].is_word() ||
           root.children[0].tokens[0].value != "NOT" {
            return Err(HandlerErr::origin(format!("Invalid NOT: {}", root)));
        }
        Ok(())
    }
  
    fn print(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;
      
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
        wrap!(writer.w(format!("{}!", spacer)));

        match root.children[1].kind() {
            ZilNodeType::Routine => wrap!(R::print(&root.children[1], 0, &mut writer)),
            ZilNodeType::Word => wrap!(W::print(&root.children[1], 0, &mut writer)),
            _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown ZilNodeType in NOT"))),
        };
      
        Ok(())
    }
}