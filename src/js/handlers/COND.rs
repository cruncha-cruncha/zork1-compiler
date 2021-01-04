use std::fs::File;
use std::io;

use crate::js::handlers::generic_tokens::*;
use crate::js::node::*;
use crate::js::custom_buf_writer::*;

pub struct COND {}

impl HandleJS for COND {
    fn print (root: &JSNode, mut writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> {
        writer.w("(() => {\n")?;
        for i in 0..root.children.len() {
            writer.w("if (\n")?;
            match root.children[i].children[0].kind {
                JSNodeType::Routine => R::print(&root.children[i].children[0], &mut writer),
                JSNodeType::Word => W::print(&root.children[i].children[0], &mut writer),
                _ => panic!()
            }?;
            writer.w(") {\n")?;
            for j in 1..root.children[i].children.len() {
                if j+1 == root.children[i].children.len() {
                    writer.w("return (\n")?;
                }
                match root.children[i].children[j].kind {
                    JSNodeType::Routine => R::print(&root.children[i].children[j], &mut writer),
                    JSNodeType::Word | JSNodeType::Int => W::print(&root.children[i].children[j], &mut writer),
                    _ => panic!()
                }?;
                if j+1 == root.children[i].children.len() {
                    writer.w(")")?;
                }
                writer.w(";\n")?;
            }
            writer.w("}")?;
            if (i+1) < root.children.len() {
                writer.w(" else ")?;
            }
        }

        writer.w(" else {\nreturn 0;\n}})()")?;

        //writer.w("})()")?;

        Ok(())
    }
}