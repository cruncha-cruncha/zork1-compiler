use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::zil::ast::*;

use crate::js::handle::generic_tokens::*;

pub fn handle_ROUTINE(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
  if root.children.len() < 4 || !root.children[1].is_word() || !root.children[2].is_grouping() {
      return Err(());
  }

  let spacer = (0..indent).map(|_| "  ").collect::<String>();
  writer.write(format!("{0}function ", spacer).as_bytes());
  handle_w(&root.children[1], 0, &mut writer)?;
  writer.write(b"(");

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
                  return Err(());
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
                  _ => return Err(()),
              };
          },
          NodeType::Word => {
              match arg_state {
                  ArgState::INITIAL => param_buf.push(&tmp),
                  ArgState::OPTIONAL => optional_param_buf.push(&tmp),
                  ArgState::AUX => body_buf.push(&tmp),
              };
          }
          _ => return Err(()),
      };
  }

  if param_buf.len() > 0 {
      for i in 0..param_buf.len() {
          match param_buf[i].kind() {
              NodeType::Grouping => handle_w(&param_buf[i].children[0], 0, &mut writer),
              NodeType::Word => handle_w(&param_buf[i], 0, &mut writer),
              _ => Err(()),
          }?;
          if i+1 < param_buf.len() {
              writer.write(b", ");
          }
      }
  }

  if optional_param_buf.len() > 0 {
      if param_buf.len() > 0 {
          writer.write(b", ");
      }
      for i in 0..optional_param_buf.len() {
          match optional_param_buf[i].kind() {
              NodeType::Grouping => {
                  handle_w(&optional_param_buf[i].children[0], 0, &mut writer)?;
                  writer.write(b" = ");
                  match optional_param_buf[i].children[1].kind() {
                      NodeType::Routine => handle_r(&optional_param_buf[i].children[1], 0, &mut writer),
                      NodeType::Text => handle_t(&optional_param_buf[i].children[1], 0, &mut writer),
                      NodeType::Word => handle_w(&optional_param_buf[i].children[1], 0, &mut writer),
                      _ => Err(()),
                  }?;
              },
              NodeType::Word => {
                  handle_w(&optional_param_buf[i], 0, &mut writer)?;
                  writer.write(b" = null");
              },
              _ => return Err(()),
          };
          if i+1 < param_buf.len() {
              writer.write(b", ");
          }
      }
  }

  writer.write(b") {\n");

  if body_buf.len() > 0 {
      for i in 0..body_buf.len() {
          writer.write(format!("{}  let ", spacer).as_bytes());
          match body_buf[i].kind() {
              NodeType::Grouping => {
                  handle_w(&body_buf[i].children[0], 0, &mut writer)?;
                  writer.write(b" = ");
                  match body_buf[i].children[1].kind() {
                      NodeType::Routine => handle_r(&body_buf[i].children[1], 0, &mut writer),
                      NodeType::Text => handle_t(&body_buf[i].children[1], 0, &mut writer),
                      NodeType::Word => handle_w(&body_buf[i].children[1], 0, &mut writer),
                      _ => Err(()),
                  }?;
              },
              NodeType::Word => {
                  handle_w(&body_buf[i], 0, &mut writer)?;
                  writer.write(b" = null");
              },
              _ => return Err(()),
          };
          writer.write(b"\n");
      }
  }

  for i in 3..root.children.len() {
      match root.children[i].kind() {
          NodeType::Routine => handle_r(&root.children[i], indent+1, &mut writer),
          _ => Err(()),
      }?;
      writer.write(b"\n");
  }

  writer.write(format!("{}}}\n\n", spacer).as_bytes());

  Ok(())
}