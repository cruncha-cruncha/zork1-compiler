use std::fs::File;

use crate::zil::contracts::*;
use crate::js::handlers::generic_tokens::*;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

pub struct REPEAT {}

impl HandleJS for REPEAT {
    fn validate (root: &ZilNode) -> Result<(), HandlerErr> {
        if !root.is_routine() ||
           root.children.len() < 3 ||
           !root.children[0].is_word() ||
           root.children[0].tokens[0].value != "REPEAT" ||
           !root.children[1].is_grouping() ||
           root.children[1].children.len() != 0 {
            return Err(HandlerErr::origin(format!("Invalid REPEAT: {}", root)));
        }
        Ok(())
    }
  
    fn print(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;
      
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
        wrap!(writer.w(format!("{}(() => {{\n", spacer)));
        wrap!(writer.w(format!("{}  while (true) {{\n", spacer)));

        for i in 2..root.children.len() {
            wrap!(R::print(&root.children[i], indent+2, &mut writer));
            wrap!(writer.w("\n"));
        }

        wrap!(writer.w(format!("{}  }}\n", spacer)));
        wrap!(writer.w(format!("{}}})()\n", spacer)));

        Ok(())
    }
}