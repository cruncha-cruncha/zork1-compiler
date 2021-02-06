use std::fs::File;
use std::io;

use crate::js::handlers::generic_tokens::*;
use crate::js::node::*;
use crate::js::custom_buf_writer::*;

pub struct REPEAT {}

impl HandleJS for REPEAT {
    fn print (root: &JSNode, mut writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> {
        writer.w("(() => {\n")?;
        writer.w("while (true) {\n")?;

        for i in 2..root.children.len() {
            R::print(&root.children[i], &mut writer)?;
            writer.w("\n")?;
        }

        writer.w("{}  }}\n")?;
        writer.w(")()\n")?;

        Ok(())
    }
}