use std::fs::File;
use std::io::BufWriter;

use crate::zil::ast::*;

pub fn parse(root: &Node, mut writer: BufWriter<File>) -> Result<(), ()> {
    for i in 0..root.children.len() {
        if !root.children[i].is_routine() {
            return Err(());
        } else {
            crate::js::handle::generic_tokens::handle_r(&root.children[i], 0, &mut writer)?;
        }
    }

    Ok(())
}