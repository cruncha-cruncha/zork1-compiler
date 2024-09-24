use crate::{
    js::write_output::OutputNode,
    stats::{
        helpers::{
            get_token_as_number, get_token_as_text, get_token_as_word, num_children_more_than,
        },
        routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use super::set_var::Scope;

pub struct Tell {
    pub text: Vec<OutputNode>,
    pub text_types: Vec<ReturnValType>,
    pub cr: bool,
}

impl Tell {
    pub fn new() -> Self {
        Self {
            text: Vec::new(),
            text_types: Vec::new(),
            cr: false,
        }
    }
}

impl HasReturnType for Tell {
    fn return_type(&self) -> Vec<ReturnValType> {
        vec![ReturnValType::None]
    }
}

impl CanValidate for Tell {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children_more_than(n, 1)?;

        v.expect_vals(vec![
            ReturnValType::Boolean,
            ReturnValType::Number,
            ReturnValType::Text,
        ]);

        for (i, child) in n.children.iter().skip(1).enumerate() {
            match child.node_type {
                ZilNodeType::Token(TokenType::Text) => {
                    let text = get_token_as_text(child).unwrap();
                    self.text.push(OutputNode::Text(text));
                    self.text_types.push(ReturnValType::Text);
                }
                ZilNodeType::Token(TokenType::Number) => {
                    let number = get_token_as_number(child).unwrap();
                    self.text.push(OutputNode::Number(number));
                    self.text_types.push(ReturnValType::Number);
                }
                ZilNodeType::Token(TokenType::Word) => {
                    let word = get_token_as_word(child).unwrap();
                    if (i + 1 == n.children.len() - 1) && (word == "CR") {
                        self.cr = true;
                        continue;
                    }
                    if let Some(return_type) = v.has_local_var(&word) {
                        match return_type {
                            ReturnValType::Number => {
                                self.text.push(OutputNode::Variable(Scope::Local(word)));
                                self.text_types.push(ReturnValType::Number);
                            }
                            ReturnValType::Text => {
                                self.text.push(OutputNode::Variable(Scope::Local(word)));
                                self.text_types.push(ReturnValType::Text);
                            }
                            _ => {
                                return Err(format!(
                                    "Unexpected return type: {:?}\n{}",
                                    return_type,
                                    format_file_location(&child)
                                ));
                            }
                        }
                    } else if v.is_global(&word) {
                        self.text.push(OutputNode::Variable(Scope::Global(word)));
                        self.text_types.push(ReturnValType::Number);
                    } else {
                        return Err(format!(
                            "Variable {} not found in local or global symbol table\n{}",
                            word,
                            format_file_location(&child)
                        ));
                    }
                }
                ZilNodeType::Cluster => match v.validate_cluster(child) {
                    Ok(return_type) => match v.take_last_writer() {
                        Some(w) => {
                            self.text.push(OutputNode::Writer(w));
                            self.text_types.push(return_type);
                        }
                        None => unreachable!(),
                    },
                    Err(e) => return Err(e),
                },
                _ => {
                    return Err(format!(
                        "Unexpected node type: {:?}\n{}",
                        child.node_type,
                        format_file_location(&child)
                    ));
                }
            }
        }

        Ok(())
    }
}
