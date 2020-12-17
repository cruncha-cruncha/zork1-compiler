use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::zil::ast::*;

use crate::js::handle::generic_tokens::*;

pub fn handle_OBJECT(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
  if root.children.len() < 2 {
      return Err(())
  }

  let spacer = (0..indent).map(|_| "  ").collect::<String>();
  writer.write(format!("{}let ", spacer).as_bytes());
  handle_w(&root.children[1], 0, &mut writer)?;
  writer.write(b" = {\n");

  for i in 2..root.children.len() {
      if !root.children[i].is_grouping() ||
         root.children[i].children.len() < 2 ||
         !root.children[i].children[0].is_word() {
          return Err(());
      }

      match &root.children[i].children[0].tokens[0].value[..] {
          "TEXT" | "DESC" | "LDESC" | "FDESC" | "ACTION" | "DESCFCN" => return_string(&root.children[i], indent+1, &mut writer),
          "CAPACITY" | "SIZE" | "VALUE" | "TVALUE" => return_int(&root.children[i], indent+1, &mut writer),
          "SYNONYM" | "ADJECTIVE" => return_string_array(&root.children[i], indent+1, &mut writer),
          "FLAGS" | "VTYPE" => mut_bools(&root.children[i], indent+1, &mut writer),
          "STRENGTH" => mut_int(&root.children[i], indent+1, &mut writer),
          "IN" => mut_string(&root.children[i], indent+1, &mut writer),
          _ => Err(()),
      }?;
  }

  writer.write(format!("{}}};\n\n", spacer).as_bytes());

  Ok(())
}

fn return_string(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    let spacer = (0..indent).map(|_| "  ").collect::<String>();

    writer.write(format!("{}", spacer).as_bytes());
    handle_w(&root.children[0], 0, &mut writer)?;
    writer.write(b": () => ");

    match root.children[1].kind() {
        NodeType::Text => {
            handle_t(&root.children[1], 0, &mut writer)?;
            Ok(())
        },
        NodeType::Word => {
            writer.write(b"\"");
            handle_w(&root.children[1], 0, &mut writer)?;
            writer.write(b"\"");
            Ok(())
        },
        _ => Err(()),
    }?;

    writer.write(b",\n");

    Ok(())
}

fn return_string_array(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    let spacer = (0..indent).map(|_| "  ").collect::<String>();

    writer.write(format!("{}", spacer).as_bytes());
    handle_w(&root.children[0], 0, &mut writer)?;
    writer.write(format!(": () => [").as_bytes());

    for i in 1..root.children.len() {
        match root.children[i].kind() {
            NodeType::Word => {
                writer.write(b"\"");
                handle_w(&root.children[i], 0, &mut writer)?;
                writer.write(b"\"");
                Ok(())
            },
            _ => Err(()),
        }?;

        if i+1 < root.children.len() {
            writer.write(b", ");
        }
    }

    writer.write(b"],\n");

    Ok(()) 
}

fn return_int(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    let spacer = (0..indent).map(|_| "  ").collect::<String>();

    writer.write(format!("{}", spacer).as_bytes());
    handle_w(&root.children[0], 0, &mut writer)?;
    writer.write(b": () => ");

    match root.children[1].kind() {
        NodeType::Word => {
            // try to parse int? turbofish?
            handle_w(&root.children[1], 0, &mut writer)?;
            Ok(())
        },
        _ => Err(()),
    }?;

    writer.write(b",\n");

    Ok(())
}

fn mut_string(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    let spacer = (0..indent).map(|_| "  ").collect::<String>();

    writer.write(format!("{}", spacer).as_bytes());
    handle_w(&root.children[0], 0, &mut writer);
    writer.write(b": ");

    match root.children[1].kind() {
        NodeType::Text => {
            handle_t(&root.children[1], 0, &mut writer)?;
            Ok(())
        },
        NodeType::Word => {
            writer.write(b"\"");
            handle_w(&root.children[1], 0, &mut writer)?;
            writer.write(b"\"");
            Ok(())
        },
        _ => Err(()),
    }?;

    writer.write(b",\n");

    Ok(())
}

fn mut_int(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    let spacer = (0..indent).map(|_| "  ").collect::<String>();

    writer.write(format!("{}", spacer).as_bytes());
    handle_w(&root.children[0], 0, &mut writer)?;
    writer.write(b": ");

    match root.children[1].kind() {
        NodeType::Word => {
            // try to parse int? turbofish?
            handle_w(&root.children[1], 0, &mut writer)?;
            Ok(())
        },
        _ => Err(()),
    }?;

    writer.write(b",\n");

    Ok(())
}

fn mut_bools(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    let spacer = (0..indent).map(|_| "  ").collect::<String>();

    writer.write(format!("{}", spacer).as_bytes());
    handle_w(&root.children[0], 0, &mut writer)?;
    writer.write(format!(": {{ ").as_bytes());

    for i in 1..root.children.len() {
        match root.children[i].kind() {
            NodeType::Word => {
                handle_w(&root.children[i], 0, &mut writer)?;
                writer.write(b": true");
                Ok(())
            },
            _ => Err(()),
        }?;

        if i+1 < root.children.len() {
            writer.write(b", ");
        }
    }

    writer.write(b" },\n");

    Ok(()) 
}