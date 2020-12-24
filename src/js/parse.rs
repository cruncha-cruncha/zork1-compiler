use std::fs::File;

use crate::zil::contracts::*;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

// I don't like this, it's ugly
// I think we should transform the zil ast into a js ast, and then print

pub fn parse(root: &ZilNode, mut writer: CustomBufWriter<File>) -> Result<(), OutputErr> {
    for i in 0..root.children.len() {
        if !root.children[i].is_routine() {
            return Err(OutputErr::from(HandlerErr::origin("top-level child is not a routine")));
        } else {
            crate::js::handlers::generic_tokens::R::print(&root.children[i], 0, &mut writer)?;
        }
    }

    Ok(())
}

