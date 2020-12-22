use std::fs::File;

use crate::zil::ast::{Node, NodeType};
use crate::js::handlers::generic_tokens::*;
use crate::js::helpers::is_int;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

pub struct OBJECT {}

impl HandleJS for OBJECT {
    fn validate (root: &Node) -> Result<(), HandlerErr> {
        if root.children.len() < 2 {
            return Err(HandlerErr::origin(format!("Invalid OBJECT: {}", root)));
        }
        Ok(())
    }
  
    fn print(root: &Node, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;
      
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
        wrap!(writer.w(format!("{}let ", spacer)));
        wrap!(W::print(&root.children[1], 0, &mut writer));
        wrap!(writer.w(" = {\n"));

        for i in 2..root.children.len() {
            Self::validate_sub_grouping(&root.children[i])?;

            match &root.children[i].children[0].tokens[0].value[..] {
                "TEXT" | "DESC" | "LDESC" | "FDESC" | "ACTION" | "DESCFCN" => wrap!(Self::return_string(&root.children[i], indent+1, &mut writer)),
                "CAPACITY" | "SIZE" | "VALUE" | "TVALUE" => wrap!(Self::return_int(&root.children[i], indent+1, &mut writer)),
                "SYNONYM" | "ADJECTIVE" => wrap!(Self::return_string_array(&root.children[i], indent+1, &mut writer)),
                "FLAGS" | "VTYPE" => wrap!(Self::mut_bools(&root.children[i], indent+1, &mut writer)), // not sure if should be mutable
                "STRENGTH" => wrap!(Self::mut_int(&root.children[i], indent+1, &mut writer)), // not sure if should be mutable
                // Look up <OBJECT ROOMS ... >
                // the IN does not look like other INs
                "IN" => wrap!(Self::mut_string(&root.children[i], indent+1, &mut writer)),
                _ => return Err(OutputErr::from(HandlerErr::origin("Unknown sub grouping in OBJECT"))),
            };
        }

        wrap!(writer.w(format!("{}}};\n\n", spacer)));

        Ok(())
    }
}

impl OBJECT {
    fn validate_sub_grouping(root: &Node) -> Result<(), HandlerErr> {
        if !root.is_grouping() ||
           root.children.len() < 2 ||
           !root.children[0].is_word() {
            return Err(HandlerErr::origin(format!("Invalid OBJECT sub grouping: {}", root)));
        }
        Ok(())
    }

    fn return_string(root: &Node, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
    
        wrap!(writer.w(format!("{}", spacer)));
        wrap!(W::print(&root.children[0], 0, &mut writer));
        wrap!(writer.w(": () => "));
    
        match root.children[1].kind() {
            NodeType::Text => wrap!(T::print(&root.children[1], 0, &mut writer)),
            NodeType::Word => wrap!(W::print_with_quotes(&root.children[1], 0, &mut writer)),
            _ => return Err(OutputErr::from(HandlerErr::origin("Cannot handle unknown NodeType in OBJECT sub group 'return_string'"))),
        };
    
        wrap!(writer.w(",\n"));
    
        Ok(())
    }

    fn return_string_array(root: &Node, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
    
        wrap!(writer.w(format!("{}", spacer)));
        wrap!(W::print(&root.children[0], 0, &mut writer));
        wrap!(writer.w(format!(": () => [")));
    
        for i in 1..root.children.len() {
            match root.children[i].kind() {
                NodeType::Text => wrap!(T::print(&root.children[1], 0, &mut writer)),
                NodeType::Word => wrap!(W::print_with_quotes(&root.children[i], 0, &mut writer)),
                _ => return Err(OutputErr::from(HandlerErr::origin("Cannot handle unknown NodeType in OBJECT sub group 'return_string_array'"))),
            };
    
            if i+1 < root.children.len() {
                wrap!(writer.w(", "));
            }
        }
    
        wrap!(writer.w("],\n"));
    
        Ok(()) 
    }

    fn return_int(root: &Node, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
    
        wrap!(writer.w(format!("{}", spacer)));
        wrap!(W::print(&root.children[0], 0, &mut writer));
        wrap!(writer.w(": () => "));
    
        match root.children[1].kind() {
            NodeType::Word => {
                match is_int(&root.children[1]) {
                  true => wrap!(W::print(&root.children[1], 0, &mut writer)),
                  false => return Err(OutputErr::from(HandlerErr::origin(format!("Trying to parse not-an-int in OBJECT sub group 'return_int': {}", root.children[1]))))
                };
            },
            _ => return Err(OutputErr::from(HandlerErr::origin("Cannot handle unknown NodeType in OBJECT sub group 'return_int'"))),
        };
    
        wrap!(writer.w(",\n"));
    
        Ok(())
    }

    fn mut_string(root: &Node, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
    
        wrap!(writer.w(format!("{}", spacer)));
        wrap!(W::print(&root.children[0], 0, &mut writer));
        wrap!(writer.w(": "));
    
        match root.children[1].kind() {
            NodeType::Text => wrap!(T::print(&root.children[1], 0, &mut writer)),
            NodeType::Word => wrap!(W::print_with_quotes(&root.children[1], 0, &mut writer)),
            _ => return Err(OutputErr::from(HandlerErr::origin("Cannot handle unknown NodeType in OBJECT sub group 'mut_string'"))),
        };
    
        wrap!(writer.w(",\n"));
    
        Ok(())
    }
    
    fn mut_int(root: &Node, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
    
        wrap!(writer.w(format!("{}", spacer)));
        wrap!(W::print(&root.children[0], 0, &mut writer));
        wrap!(writer.w(": "));
    
        match root.children[1].kind() {
            NodeType::Word => {
                match is_int(&root.children[1]) {
                  true => wrap!(W::print(&root.children[1], 0, &mut writer)),
                  false => return Err(OutputErr::from(HandlerErr::origin(format!("Trying to parse not-an-int in OBJECT sub group 'mut_int': {}", root.children[1]))))
                };
            },
            _ => return Err(OutputErr::from(HandlerErr::origin("Cannot handle unknown NodeType in OBJECT sub group 'mut_int'"))),
        };
    
        wrap!(writer.w(",\n"));
    
        Ok(())
    }
    
    fn mut_bools(root: &Node, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
    
        wrap!(writer.w(format!("{}", spacer)));
        wrap!(W::print(&root.children[0], 0, &mut writer));
        wrap!(writer.w(format!(": {{ ")));
    
        for i in 1..root.children.len() {
            match root.children[i].kind() {
                NodeType::Word => {
                    wrap!(W::print(&root.children[i], 0, &mut writer));
                    wrap!(writer.w(": true"));
                },
                _ => return Err(OutputErr::from(HandlerErr::origin("Cannot handle unknown NodeType in OBJECT sub group 'mut_bools'"))),
            };
    
            if i+1 < root.children.len() {
                wrap!(writer.w(", "));
            }
        }
    
        wrap!(writer.w(" },\n"));
    
        Ok(()) 
    }
}

