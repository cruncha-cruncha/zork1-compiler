use std::fs::File;
use std::io;

use crate::js::handlers::generic_tokens::*;
use crate::js::node::*;
use crate::js::custom_buf_writer::*;

pub struct OBJECT {}

impl HandleJS for OBJECT {
    fn print (root: &JSNode, mut writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> {      
        writer.w("let ")?;
        W::print(&root.children[0], &mut writer)?;
        writer.w(" = {\n")?;

        for i in 1..root.children.len() {
            match &root.children[i].children[0].value[..] {
                "TEXT" | "DESC" | "LDESC" | "FDESC" | "DESCFCN" | "IN" | "ACTION" => Self::format_string(&root.children[i], &mut writer)?,
                "CAPACITY" | "SIZE" | "VALUE" | "TVALUE" | "STRENGTH" => Self::format_int(&root.children[i], &mut writer)?,
                "SYNONYM" | "ADJECTIVE" => Self::format_array_of_strings(&root.children[i], &mut writer)?,
                "FLAGS" | "VTYPE" => Self::format_flag_object(&root.children[i], &mut writer)?,
                _ => (),
            };
        }

        writer.w("};\n")?;

        Ok(())
    }
}

impl OBJECT {
    pub fn format_string(root: &JSNode, mut writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> {    
        W::print(&root.children[0], &mut writer)?;
        writer.w(": ")?;
    
        match root.children[1].kind {
            JSNodeType::Text => T::print(&root.children[1], &mut writer)?,
            JSNodeType::Word => W::print_with_quotes(&root.children[1], &mut writer)?,
            _ => (),
        };
    
        writer.w(",\n")?;
    
        Ok(())
    }

    pub fn format_array_of_strings(root: &JSNode, mut writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> {
        W::print(&root.children[0], &mut writer)?;
        writer.w(": [")?;
    
        for i in 1..root.children.len() {
            match root.children[i].kind {
                JSNodeType::Text => T::print(&root.children[i], &mut writer)?,
                JSNodeType::Word => W::print_as_text(&root.children[i], &mut writer)?,
                _ => (),
            };
    
            if i+1 < root.children.len() {
                writer.w(", ")?;
            }
        }
    
        writer.w("],\n")?;
    
        Ok(()) 
    }

    pub fn format_int(root: &JSNode, mut writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> {    
        W::print(&root.children[0], &mut writer)?;
        writer.w(": ")?;
        W::print(&root.children[1], &mut writer)?;
        writer.w(",\n")?;
    
        Ok(())
    }
    
    pub fn format_flag_object(root: &JSNode, mut writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> {
        W::print(&root.children[0], &mut writer)?;
        writer.w(": { ")?;
    
        for i in 1..root.children.len() {
            W::print(&root.children[i], &mut writer)?;
            writer.w(": true")?;

            if i+1 < root.children.len() {
                writer.w(", ")?;
            }
        }
    
        writer.w(" },\n")?;
    
        Ok(()) 
    }
}