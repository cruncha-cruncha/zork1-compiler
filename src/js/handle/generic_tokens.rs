use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::zil::ast::*;

use crate::js::helpers::*;

// <ROOM ... >
// <INSERT-FILE ... > // literally just "import ..."

pub fn handle_r(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    if !root.is_routine() {
        return Err(());
    }

    let spacer = (0..indent).map(|_| "  ").collect::<String>(); 

    if root.children.len() <= 0 {
        writer.write(format!("{}null", spacer).as_bytes());
        return Ok(());
    }

    match &root.children[0].tokens[0].value[..] {
        "COND" => crate::js::handle::COND::handle_COND(&root, indent, &mut writer),
        "ROUTINE" => crate::js::handle::ROUTINE::handle_ROUTINE(&root, indent, &mut writer),
        "REPEAT" => crate::js::handle::REPEAT::handle_REPEAT(&root, indent, &mut writer),
        "OBJECT" => crate::js::handle::OBJECT::handle_OBJECT(&root, indent, &mut writer),
        "TELL" => crate::js::handle::TELL::handle_TELL(&root, indent, &mut writer),
        "SET" => crate::js::handle::SET::handle_SET(&root, indent, &mut writer),
        "EQUAL?" | "==?" | "=?" => crate::js::handle::equals::handle_equals(&root, indent, &mut writer),
        "AND" => crate::js::handle::AND::handle_AND(&root, indent, &mut writer),
        "OR" => crate::js::handle::OR::handle_OR(&root, indent, &mut writer),
        "NOT" => crate::js::handle::NOT::handle_NOT(&root, indent, &mut writer),
        "+" => crate::js::handle::add::handle_add(&root, indent, &mut writer),
        "-" => crate::js::handle::subtract::handle_subtract(&root, indent, &mut writer),
        "*" => crate::js::handle::multiply::handle_multiply(&root, indent, &mut writer),
        "/" => crate::js::handle::divide::handle_divide(&root, indent, &mut writer),
        _ => {
            writer.write(format!("{}", spacer).as_bytes());
            handle_w(&root.children[0], 0, &mut writer);
            writer.write(b"(");
            for i in 1..root.children.len() {
                match root.children[i].kind() {
                    NodeType::Routine => handle_r(&root.children[i], 0, &mut writer),
                    NodeType::Grouping => handle_g(&root.children[i], 0, &mut writer),
                    NodeType::Text => handle_t(&root.children[i], 0, &mut writer),
                    NodeType::Word => handle_w(&root.children[i], 0, &mut writer),
                    _ => Err(()),
                }?;
                if i+1 < root.children.len() {
                    writer.write(b", ");
                }
            }
            writer.write(b")");
            Ok(())
        }
    }?;

    Ok(())
}

pub fn handle_g(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    if !root.is_grouping() {
        return Err(());
    }

    let spacer = (0..indent).map(|_| "  ").collect::<String>(); 
    writer.write(format!("{}(", spacer).as_bytes());

    for i in 0..root.children.len() {
        match root.children[i].kind() {
            NodeType::Routine => handle_r(&root.children[i], 0, &mut writer),
            NodeType::Grouping => handle_g(&root.children[i], 0, &mut writer),
            NodeType::Text => handle_t(&root.children[i], 0, &mut writer),
            NodeType::Word => handle_w(&root.children[i], 0, &mut writer),
            _ => Err(()),
        }?;
        if i+1 < root.children.len() {
            writer.write(b" ");
        }
    }

    writer.write(b")");

    Ok(())
}

pub fn handle_t(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    if !root.is_text() {
        return Err(());
    }

    let spacer = (0..indent).map(|_| "  ").collect::<String>(); 
    writer.write(format!("{}{}", spacer, format_string(&root)?).as_bytes());

    Ok(())
}

pub fn handle_w(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    if !root.is_word() {
        return Err(());
    }

    let spacer = (0..indent).map(|_| "  ").collect::<String>(); 
    match &root.tokens[0].value[..] {
        "T" => writer.write(format!("{}true", spacer).as_bytes()),
        v => {
            writer.write(format!("{}{}", spacer, format_keyword(&root)?).as_bytes())
        },
    };

    Ok(())
}