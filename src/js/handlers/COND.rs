use std::fs::File;

use crate::zil::contracts::*;
use crate::js::handlers::generic_tokens::*;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

pub struct COND {}

impl HandleJS for COND {
    fn validate (root: &ZilNode) -> Result<(), HandlerErr> {
        if !root.is_routine() ||
           root.children.len() < 2 ||
           !root.children[0].is_word() ||
           root.children[0].tokens[0].value != "COND" ||
           !root.children[1].is_grouping() || 
           root.children[1].children.len() < 2 {
            return Err(HandlerErr::origin(format!("Invalid COND: {}", root)));
        }
        Ok(())
    }

    fn print(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;

        let spacer = (0..indent).map(|_| "  ").collect::<String>();
        wrap!(writer.w(format!("{}if ", spacer)));

        for g in 1..root.children.len() {
            wrap!(writer.w("("));
            match root.children[g].children[0].kind() {
                ZilNodeType::Routine => wrap!(R::print(&root.children[g].children[0], 0, &mut writer)),       
                ZilNodeType::Word => wrap!(W::print(&root.children[g].children[0], 0, &mut writer)),
                _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown ZilNodeType in conditional of COND"))),
            };
            wrap!(writer.w(") {\n"));
        
            for i in 1..root.children[g].children.len() {
                match root.children[g].children[i].kind() {
                    ZilNodeType::Routine => wrap!(R::print(&root.children[g].children[i], indent+1, &mut writer)),
                    ZilNodeType::Word => wrap!(W::print(&root.children[g].children[i], indent+1, &mut writer)),
                    _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown ZilNodeType in body of COND"))),
                };
                wrap!(writer.w("\n"));
            }
            if g+1 < root.children.len() {
                wrap!(writer.w(format!("{}}} else if ", spacer)));
            }
        }

        wrap!(writer.w(format!("{}}}\n", spacer)));
        
        Ok(())
    }
}