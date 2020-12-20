use std::fs::File;

use crate::zil::ast::Node;

use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;


pub fn parse(root: &Node, mut writer: CustomBufWriter<File>) -> Result<(), OutputErr> {
    for i in 0..root.children.len() {
        if !root.children[i].is_routine() {
            return Err(OutputErr::from(HandlerErr::origin("top-level child is not a routine")));
        } else {
            crate::js::handlers::generic_tokens::R::print(&root.children[i], 0, &mut writer)?;
        }
    }

    Ok(())
}





