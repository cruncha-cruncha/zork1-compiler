use std::fs::File;
use std::io;

use crate::js::node::*;
use crate::js::handlers::generic_tokens::*;
use crate::js::custom_buf_writer::*;

// I don't like this, it's ugly
// I think we should transform the zil ast into a js ast, and then print

pub fn parse(root: &JSNode, mut writer: CustomBufWriter<File>) -> Result<(), io::Error> {
    for i in 0..root.children.len() {
        R::print(&root.children[i], &mut writer)?;
    }

    Ok(())
}

