use std::fs::File;

use crate::zil::contracts::*;
use crate::js::handlers::generic_tokens::*;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

pub struct ROOM {}

impl HandleJS for ROOM {
    fn validate (root: &ZilNode) -> Result<(), HandlerErr> {
        if !root.is_routine() ||
           root.children.len() < 2 ||
           !root.children[0].is_word() ||
           root.children[0].tokens[0].value != "ROOM" {
            return Err(HandlerErr::origin(format!("Invalid ROOM: {}", root)));
        }
        Ok(())
    }
  
    fn print(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;
      
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
        wrap!(writer.w(format!("{}let ", spacer)));
        wrap!(W::print(&root.children[1], 0, &mut writer));
        wrap!(writer.w(" = {\n"));

        for i in 2..root.children.len() {
            Self::validate_sub_grouping(&root.children[i])?;

            match &root.children[i].children[0].tokens[0].value[..] {
                "PSEUDO" => wrap!(Self::return_obj(&root.children[i], indent+1, &mut writer)),
                "DESC" | "LDESC" | "ACTION" | "IN" => wrap!(crate::js::handlers::OBJECT::OBJECT::return_string(&root.children[i], indent+1, &mut writer)),
                "GLOBAL" | "FLAGS" => wrap!(crate::js::handlers::OBJECT::OBJECT::mut_bools(&root.children[i], indent+1, &mut writer)),
                "VALUE" => wrap!(crate::js::handlers::OBJECT::OBJECT::return_int(&root.children[i], indent+1, &mut writer)),
                "NORTH" | "NE" | "EAST" | "SE" | "SOUTH" | "SW" | "WEST" | "NW" | "LAND" | "UP" | "DOWN" | "OUT" => wrap!(crate::js::handlers::subgrouping_IN::SubgroupingIN::print(&root.children[i], indent+1, &mut writer)),
                _ => return Err(OutputErr::from(HandlerErr::origin("Unknown sub grouping in ROOM"))),
            };
        }

        wrap!(writer.w(format!("{}}};\n\n", spacer)));

        Ok(())
    }
}

impl ROOM {
    pub fn validate_sub_grouping(root: &ZilNode) -> Result<(), HandlerErr> {
        if !root.is_grouping() ||
           root.children.len() < 2 ||
           !root.children[0].is_word() {
            return Err(HandlerErr::origin(format!("Invalid ROOM sub grouping: {}", root)));
        }
        Ok(())
    }

    pub fn return_obj(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
      if root.children.len()%2 != 1 {
        return Err(OutputErr::from(HandlerErr::origin("Bad number of children in ROOM sub group 'return_obj'")));
      }

      let spacer = (0..indent).map(|_| "  ").collect::<String>();
  
      wrap!(writer.w(format!("{}", spacer)));
      wrap!(W::print(&root.children[0], 0, &mut writer));
      wrap!(writer.w(": () => { return { "));

      for i in 0..(root.children.len() - 1)/2 {
        match root.children[(i*2)+1].kind() {
          ZilNodeType::Text => wrap!(T::print_as_word(&root.children[(i*2)+1], 0, &mut writer)),
          ZilNodeType::Word => wrap!(W::print(&root.children[(i*2)+1], 0, &mut writer)),
          _ => return Err(OutputErr::from(HandlerErr::origin("Cannot handle unknown ZilNodeType for key in ROOM sub group 'return_obj'"))),
        };

        wrap!(writer.w(": "));

        match root.children[(i*2)+2].kind() {
          ZilNodeType::Text => wrap!(T::print(&root.children[(i*2)+2], 0, &mut writer)),
          ZilNodeType::Word => wrap!(W::print(&root.children[(i*2)+2], 0, &mut writer)),
          _ => return Err(OutputErr::from(HandlerErr::origin("Cannot handle unknown ZilNodeType for value in ROOM sub group 'return_obj'"))),
        };

        if ((i+1)*2)+1 < root.children.len() {
          wrap!(writer.w(","));
        } 
        wrap!(writer.w(" "));
      }
  
      wrap!(writer.w("}},\n"));
  
      Ok(())
    }

    pub fn mut_fn(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
      let spacer = (0..indent).map(|_| "  ").collect::<String>();
  
      wrap!(writer.w(format!("{}", spacer)));
      wrap!(W::print(&root.children[0], 0, &mut writer));
      wrap!(writer.w(": "));
      wrap!(W::print(&root.children[1], 0, &mut writer));
      wrap!(writer.w(",\n"));
  
      Ok(())
  }
}

