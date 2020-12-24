use std::fs::File;

use crate::zil::contracts::*;
use crate::js::handlers::generic_tokens::*;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

pub struct ROUTINE {}

impl HandleJS for ROUTINE {
    fn validate (root: &ZilNode) -> Result<(), HandlerErr> {
        if root.children.len() < 4 ||
           !root.children[0].is_word() ||
           !root.children[1].is_word() ||
           !root.children[2].is_grouping() ||
           root.children[0].tokens[0].value != "ROUTINE" {
            return Err(HandlerErr::origin(format!("Invalid ROUTINE: {}", root)));
        }
        Ok(())
    }
  
    fn print(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
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
      
        let mut param_buf: Vec<&ZilNode> = Vec::new();
        let mut optional_param_buf: Vec<&ZilNode> = Vec::new();
        let mut body_buf: Vec<&ZilNode> = Vec::new();
        let mut arg_state = ArgState::INITIAL;
        for i in 0..root.children[2].children.len() {
            let tmp = &root.children[2].children[i];
            match tmp.kind() {
                ZilNodeType::Grouping => {
                    if tmp.children.len() != 2 || !tmp.children[0].is_word() {
                        return Err(OutputErr::from(HandlerErr::origin("Invalid grouping in ROUTINE")));
                    }
                    match arg_state {
                        ArgState::INITIAL => param_buf.push(&tmp),
                        ArgState::OPTIONAL => optional_param_buf.push(&tmp),
                        ArgState::AUX => body_buf.push(&tmp),
                    };
                },
                ZilNodeType::Text => {
                    match &tmp.tokens[0].value[..] {
                        "AUX" => arg_state = ArgState::AUX,
                        "OPTIONAL" => arg_state = ArgState::OPTIONAL,
                        _ => return Err(OutputErr::from(HandlerErr::origin("Cannot handle unknown Text ZilNodeType in ROUTINE"))),
                    };
                },
                ZilNodeType::Word => {
                    match arg_state {
                        ArgState::INITIAL => param_buf.push(&tmp),
                        ArgState::OPTIONAL => optional_param_buf.push(&tmp),
                        ArgState::AUX => body_buf.push(&tmp),
                    };
                }
                _ => return Err(OutputErr::from(HandlerErr::origin("Cannot handle unknown ZilNodeType in ROUTINE"))),
            };
        }
      
        if param_buf.len() > 0 {
            for i in 0..param_buf.len() {
                match param_buf[i].kind() {
                    ZilNodeType::Grouping => wrap!(W::print(&param_buf[i].children[0], 0, &mut writer)),
                    ZilNodeType::Word => wrap!(W::print(&param_buf[i], 0, &mut writer)),
                    _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown ZilNodeType in param_buf in ROUTINE"))),
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
                    ZilNodeType::Grouping => {
                        wrap!(W::print(&optional_param_buf[i].children[0], 0, &mut writer));
                        wrap!(writer.w(" = "));
                        match optional_param_buf[i].children[1].kind() {
                            ZilNodeType::Routine => wrap!(R::print(&optional_param_buf[i].children[1], 0, &mut writer)),
                            ZilNodeType::Text => wrap!(T::print(&optional_param_buf[i].children[1], 0, &mut writer)),
                            ZilNodeType::Word => wrap!(W::print(&optional_param_buf[i].children[1], 0, &mut writer)),
                            _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown ZilNodeType in grouping in optional_param_buf in ROUTINE"))),
                        };
                    },
                    ZilNodeType::Word => {
                        wrap!(W::print(&optional_param_buf[i], 0, &mut writer));
                        wrap!(writer.w(" = null"));
                    },
                    _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown ZilNodeType in optional_param_buf in ROUTINE"))),
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
                    ZilNodeType::Grouping => {
                        wrap!(W::print(&body_buf[i].children[0], 0, &mut writer));
                        wrap!(writer.w(" = "));
                        match body_buf[i].children[1].kind() {
                            ZilNodeType::Routine => wrap!(R::print(&body_buf[i].children[1], 0, &mut writer)),
                            ZilNodeType::Text => wrap!(T::print(&body_buf[i].children[1], 0, &mut writer)),
                            ZilNodeType::Word => wrap!(W::print(&body_buf[i].children[1], 0, &mut writer)),
                            _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown ZilNodeType in grouping in body_buf in ROUTINE"))),
                        };
                    },
                    ZilNodeType::Word => {
                        wrap!(W::print(&body_buf[i], 0, &mut writer));
                        wrap!(writer.w(" = null"));
                    },
                    _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown ZilNodeType in body_buf in ROUTINE"))),
                };
                wrap!(writer.w("\n"));
            }
        }
      
        for i in 3..root.children.len() {
            wrap!(R::print(&root.children[i], indent+1, &mut writer));
            wrap!(writer.w("\n"));
        }
      
        wrap!(writer.w(format!("{}}}\n\n", spacer)));
      
        Ok(())
    }
}