use std::fs::File;

use crate::zil::ast::{Node, NodeType};
use crate::js::handlers::generic_tokens::*;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

pub struct ROUTINE {}

impl HandleJS for ROUTINE {
    fn validate (root: &Node) -> Result<(), HandlerErr> {
        if root.children.len() < 4 ||
           !root.children[1].is_word() ||
           !root.children[2].is_grouping() {
            return Err(HandlerErr::origin(format!("Invalid ROUTINE: {}", root)));
        }
        Ok(())
    }
  
    fn print(root: &Node, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;
      
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
        wrap!(writer.w(format!("{0}function ", spacer)));
        wrap!(W::print(&root.children[1], 0, &mut writer));
        wrap!(writer.w("("));
      
        enum ArgState {
            INITIAL,
            OPTIONAL,
            AUX,
        }
      
        let mut param_buf: Vec<&Node> = Vec::new();
        let mut optional_param_buf: Vec<&Node> = Vec::new();
        let mut body_buf: Vec<&Node> = Vec::new();
        let mut arg_state = ArgState::INITIAL;
        for i in 0..root.children[2].children.len() {
            let tmp = &root.children[2].children[i];
            match tmp.kind() {
                NodeType::Grouping => {
                    if tmp.children.len() != 2 || !tmp.children[0].is_word() {
                        return Err(OutputErr::from(HandlerErr::origin("Invalid grouping in ROUTINE")));
                    }
                    match arg_state {
                        ArgState::INITIAL => param_buf.push(&tmp),
                        ArgState::OPTIONAL => optional_param_buf.push(&tmp),
                        ArgState::AUX => body_buf.push(&tmp),
                    };
                },
                NodeType::Text => {
                    match &tmp.tokens[0].value[..] {
                        "AUX" => arg_state = ArgState::AUX,
                        "OPTIONAL" => arg_state = ArgState::OPTIONAL,
                        _ => return Err(OutputErr::from(HandlerErr::origin("Cannot handle unknown Text NodeType in ROUTINE"))),
                    };
                },
                NodeType::Word => {
                    match arg_state {
                        ArgState::INITIAL => param_buf.push(&tmp),
                        ArgState::OPTIONAL => optional_param_buf.push(&tmp),
                        ArgState::AUX => body_buf.push(&tmp),
                    };
                }
                _ => return Err(OutputErr::from(HandlerErr::origin("Cannot handle unknown NodeType in ROUTINE"))),
            };
        }
      
        if param_buf.len() > 0 {
            for i in 0..param_buf.len() {
                match param_buf[i].kind() {
                    NodeType::Grouping => wrap!(W::print(&param_buf[i].children[0], 0, &mut writer)),
                    NodeType::Word => wrap!(W::print(&param_buf[i], 0, &mut writer)),
                    _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown NodeType in param_buf in ROUTINE"))),
                };
                if i+1 < param_buf.len() {
                    wrap!(writer.w(", "));
                }
            }
        }
      
        if optional_param_buf.len() > 0 {
            if param_buf.len() > 0 {
                wrap!(writer.w(", "));
            }
            for i in 0..optional_param_buf.len() {
                match optional_param_buf[i].kind() {
                    NodeType::Grouping => {
                        wrap!(W::print(&optional_param_buf[i].children[0], 0, &mut writer));
                        wrap!(writer.w(" = "));
                        match optional_param_buf[i].children[1].kind() {
                            NodeType::Routine => wrap!(R::print(&optional_param_buf[i].children[1], 0, &mut writer)),
                            NodeType::Text => wrap!(T::print(&optional_param_buf[i].children[1], 0, &mut writer)),
                            NodeType::Word => wrap!(W::print(&optional_param_buf[i].children[1], 0, &mut writer)),
                            _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown NodeType in grouping in optional_param_buf in ROUTINE"))),
                        };
                    },
                    NodeType::Word => {
                        wrap!(W::print(&optional_param_buf[i], 0, &mut writer));
                        wrap!(writer.w(" = null"));
                    },
                    _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown NodeType in optional_param_buf in ROUTINE"))),
                };
                if i+1 < param_buf.len() {
                    wrap!(writer.w(", "));
                }
            }
        }
      
        wrap!(writer.w(") {\n"));
      
        if body_buf.len() > 0 {
            for i in 0..body_buf.len() {
                wrap!(writer.w(format!("{}  let ", spacer)));
                match body_buf[i].kind() {
                    NodeType::Grouping => {
                        wrap!(W::print(&body_buf[i].children[0], 0, &mut writer));
                        wrap!(writer.w(" = "));
                        match body_buf[i].children[1].kind() {
                            NodeType::Routine => wrap!(R::print(&body_buf[i].children[1], 0, &mut writer)),
                            NodeType::Text => wrap!(T::print(&body_buf[i].children[1], 0, &mut writer)),
                            NodeType::Word => wrap!(W::print(&body_buf[i].children[1], 0, &mut writer)),
                            _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown NodeType in grouping in body_buf in ROUTINE"))),
                        };
                    },
                    NodeType::Word => {
                        wrap!(W::print(&body_buf[i], 0, &mut writer));
                        wrap!(writer.w(" = null"));
                    },
                    _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown NodeType in body_buf in ROUTINE"))),
                };
                wrap!(writer.w("\n"));
            }
        }
      
        for i in 3..root.children.len() {
            match root.children[i].kind() {
                NodeType::Routine => wrap!(R::print(&root.children[i], indent+1, &mut writer)),
                _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown NodeType in children in ROUTINE"))),
            };
            wrap!(writer.w("\n"));
        }
      
        wrap!(writer.w(format!("{}}}\n\n", spacer)));
      
        Ok(())
    }
}