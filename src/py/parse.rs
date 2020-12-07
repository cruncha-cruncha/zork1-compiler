use std::fs::File;
use std::io::BufWriter;

use crate::zil::ast::*;
use crate::py::handle::*;

pub fn parse(root: &Node, mut writer: BufWriter<File>) -> Result<(), ()> {
    for i in 0..root.children.len() {
        if !root.children[i].is_routine() {
            return Err(());
        } else {
            handle_r(&root.children[i], 0, &mut writer);
        }
    }

    Ok(())
}