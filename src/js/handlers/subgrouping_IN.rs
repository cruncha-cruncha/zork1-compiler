use std::fs::File;

use crate::zil::contracts::*;
use crate::js::handlers::generic_tokens::*;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

pub struct SubgroupingIN {}

// (EAST TO DAM-ROOM) -> EAST: () => to(DAM-ROOM)
// (EAST "Can't do it.") -> EAST: () => "Can't do it."
// (IN LOCAL-GLOBALS) -> IN: LOCAL-GLOBALS
impl HandleJS for SubgroupingIN {
    fn validate (root: &ZilNode) -> Result<(), HandlerErr> {
        if !root.is_grouping() ||
           root.children.len() < 2 ||
           root.children.len() > 3 ||
           !root.children[0].is_word() {
            return Err(HandlerErr::origin(format!("Invalid SubgroupingIN: {}", root)));
        }
        Ok(())
    }
  
    fn print(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;

        if root.children.len() == 2 {
            match root.children[1].kind() {
                ZilNodeType::Text => wrap!(crate::js::handlers::OBJECT::OBJECT::return_string(&root, indent, &mut writer)),
                ZilNodeType::Word => wrap!(crate::js::handlers::ROOM::ROOM::mut_fn(&root, indent, &mut writer)),
                _ => return Err(OutputErr::from(HandlerErr::origin("Cannot handle unknown ZilNodeType in SubgroupingIN of length 2")))
            };
        } else if root.children.len() == 3 {
            wrap!(Self::return_TO(&root, indent, &mut writer));
        }
      
        Ok(())
    }
}

impl SubgroupingIN {
    #[allow(non_snake_case)]
    pub fn return_TO(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        if !root.children[1].is_word() ||
           root.children[1].tokens[0].value != "TO" ||
           !root.children[2].is_word() {
            return Err(OutputErr::from(HandlerErr::origin("Invalid SubgroupingIN of length three")));
        }

        let spacer = (0..indent).map(|_| "  ").collect::<String>();
    
        wrap!(writer.w(format!("{}", spacer)));
        wrap!(W::print(&root.children[0], 0, &mut writer));
        wrap!(writer.w(format!(": () => ")));
        wrap!(W::print(&root.children[1], 0, &mut writer));
        wrap!(writer.w(format!("(")));
        wrap!(W::print(&root.children[2], 0, &mut writer));
        wrap!(writer.w("),\n"));
    
        Ok(()) 
    }
}

