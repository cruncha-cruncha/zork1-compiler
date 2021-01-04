use std::fs::File;

use crate::zil::contracts::*;
use crate::js::handlers::generic_tokens::*;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

// <TABLE (PURE) ... >
// <TABLE DEF1 0 0> // DEF1 is a TABLE as defined in a GLOBAL
// <TABLE 0 0 0 0>
// <TABLE <> <> <> <>>
// <TABLE DEF3A 0 DEF3B 0 DEF3C>

// <LTABLE 0 ... >
// <LTABLE (PURE) RIVER-1 RIVER-2 RIVER-3 RIVER-4 RIVER-5>
// <LTABLE <TABLE TROLL SWORD 1 0 TROLL-MELEE> <TABLE THIEF KNIFE 1 0 THIEF-MELEE> <TABLE CYCLOPS <> 0 0 CYCLOPS-MELEE>>>

pub struct Table {}

impl HandleJS for Table {
    fn validate (root: &ZilNode) -> Result<(), HandlerErr> {
        if !root.is_routine() ||
           root.children.len() < 2 ||
           !root.children[0].is_word() ||
           (root.children[0].tokens[0].value != "TABLE" &&
           root.children[0].tokens[0].value != "LTABLE") {
            return Err(HandlerErr::origin(format!("Invalid Table: {}", root)));
        }
        Ok(())
    }
  
    fn print(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;
      
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
        wrap!(writer.w(format!("{}[", spacer)));

        let mut start = 1;
        if root.children[1].is_grouping() {
          // ignore it ??
          start = 2;
        }
      
        for i in start..root.children.len() {
            match root.children[i].kind() {
                ZilNodeType::Routine => wrap!(R::print(&root.children[i], 0, &mut writer)),
                ZilNodeType::Text => wrap!(T::print(&root.children[i], 0, &mut writer)),
                ZilNodeType::Word => wrap!(W::print(&root.children[i], 0, &mut writer)),
                _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown ZilNodeType in TABLE"))),
            };
            if i+1 < root.children.len() {
                wrap!(writer.w(", "));
            }
        }
      
        wrap!(writer.w("]"));
      
        Ok(())
    }
}