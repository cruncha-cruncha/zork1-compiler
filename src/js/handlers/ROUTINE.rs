use std::fs::File;
use std::io;

use crate::js::node::*;
use crate::js::custom_buf_writer::*;
use crate::js::handlers::generic_tokens::*;

pub struct ROUTINE {}

impl HandleJS for ROUTINE {
    fn print (root: &JSNode, mut writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> {
        writer.w(format!("let {} = (\n", root.children[0].value))?;

        let mut optional = false;
        for i in 0..root.children[1].children.len() {
            match root.children[1].children[i].kind {
                JSNodeType::Text => optional = true,
                JSNodeType::Word => {
                    if optional {
                        writer.w(format!("{} = {}", root.children[1].children[i].children[0].value, root.children[1].children[i].children[1].value))?;
                    } else {
                        writer.w(format!("{}", root.children[1].children[i].value))?;
                    }
                    if (i+1) < root.children[1].children.len() {
                        writer.w(", ")?;
                    }
                },
                _ => panic!()
            };
        }

        writer.w(") => {\n")?;

        for i in 2..root.children.len() {
            if i+1 == root.children.len() {
                writer.w("return (\n")?;
            }
            R::print(&root.children[i], &mut writer)?;
            if i+1 == root.children.len() {
                writer.w(")")?;
            }
            writer.w(";\n")?;
        }

        writer.w("}\n")?;

        Ok(())
    }
}