use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

//use crate::zil::tokenize::*;
use crate::zil::ast::*;

pub fn handle_r(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    let spacer = (0..indent).map(|_| "  ").collect::<String>(); 

    if root.children.len() <= 0 {
        writer.write(format!("{}None", spacer).as_bytes());
        return Ok(());
    }

    if root.children[0].kind() != NodeType::Word {
        return Err(());
    }

    match &root.children[0].tokens[0].value[..] {
        "COND" => handle_r_COND(&root, indent, &mut writer),
        v => {
            writer.write(format!("{}{}(", spacer, v).as_bytes());
            for i in 1..root.children.len() {
                match root.children[i].kind() {
                    NodeType::Routine => handle_r(&root.children[i], 0, &mut writer),
                    NodeType::Grouping => handle_g(&root.children[i], 0, &mut writer),
                    NodeType::Comment => return Err(()),
                    NodeType::Text => handle_t(&root.children[i], 0, &mut writer),
                    NodeType::Word => handle_w(&root.children[i], 0, &mut writer),
                };
                if i+1 < root.children.len() {
                    writer.write(b", ");
                }
            }
            writer.write(b")");
            Ok(())
        }
    }
}

fn handle_g(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    let spacer = (0..indent).map(|_| "  ").collect::<String>(); 
    writer.write(format!("{}(", spacer).as_bytes());
    for i in 0..root.children.len() {
        match root.children[i].kind() {
            NodeType::Routine => handle_r(&root.children[i], 0, &mut writer),
            NodeType::Grouping => handle_g(&root.children[i], 0, &mut writer),
            NodeType::Comment => return Err(()),
            NodeType::Text => handle_t(&root.children[i], 0, &mut writer),
            NodeType::Word => handle_w(&root.children[i], 0, &mut writer),
        };
        if i+1 < root.children.len() {
            writer.write(b" ");
        }
    }
    writer.write(b")");
    Ok(())
}

fn handle_t(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    let spacer = (0..indent).map(|_| "  ").collect::<String>(); 
    writer.write(format!("{}\"{}\"", spacer, root.tokens[0].value).as_bytes());
    Ok(())
}

fn handle_w(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    let spacer = (0..indent).map(|_| "  ").collect::<String>(); 
    writer.write(format!("{}{}", spacer, root.tokens[0].value).as_bytes());
    Ok(())
}

fn handle_r_COND(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    if root.children.len() <= 1 ||
       !root.children[1].is_grouping() || 
       root.children[1].children.len() <= 0 {
        return Err(());
    }
    let spacer = (0..indent).map(|_| "  ").collect::<String>();
    writer.write(format!("{}if ", spacer).as_bytes());

    for g in 1..root.children.len() {
        if !root.children[g].is_grouping() ||
           root.children[g].children.len() <= 0 {
            return Err(());
        }

        match root.children[g].children[0].kind() {
            NodeType::Routine => {
                handle_r(&root.children[g].children[0], 0, &mut writer);
                writer.write(b":\n");
            },        
            NodeType::Word =>  {
                handle_w(&root.children[g].children[0], 0, &mut writer);
                writer.write(b":\n");
            },
            NodeType::Grouping | NodeType::Comment | NodeType::Text => {
                return Err(());
            }
        }
    
        for i in 1..root.children[g].children.len() {
            match root.children[g].children[i].kind() {
                NodeType::Routine => handle_r(&root.children[g].children[i], indent+1, &mut writer),
                NodeType::Word => handle_w(&root.children[g].children[i], indent+1, &mut writer),
                _ => return Err(()),
            };
        }

        writer.write(b"\n");
        if g+1 < root.children.len() {
            writer.write(format!("{}elif ", spacer).as_bytes());
        }
    }
    
    Ok(())
}