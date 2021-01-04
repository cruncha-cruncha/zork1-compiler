use std::fs::File;

use crate::zil::contracts::*;
use crate::js::handlers::generic_tokens::*;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

pub struct GLOBAL {}

impl HandleJS for GLOBAL {
    fn validate (root: &ZilNode) -> Result<(), HandlerErr> {
        if !root.is_routine() ||
           root.children.len() != 3 ||
           !root.children[0].is_word() ||
           root.children[0].tokens[0].value != "GLOBAL" {
            return Err(HandlerErr::origin(format!("Invalid GLOBAL: {}", root)));
        }
        Ok(())
    }
  
    fn print(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;
      
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
        wrap!(writer.w(format!("{}let ", spacer)));
        wrap!(W::print(&root.children[1], 0, &mut writer));
        wrap!(writer.w(" = "));
      
        match root.children[2].kind() {
            ZilNodeType::Routine => wrap!(R::print(&root.children[2], 0, &mut writer)),
            ZilNodeType::Text => wrap!(T::print(&root.children[2], 0, &mut writer)),
            ZilNodeType::Word => wrap!(W::print(&root.children[2], 0, &mut writer)),
            _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown ZilNodeType in GLOBAL"))),
        };
      
        wrap!(writer.w(";\n"));
      
        Ok(())
    }
}