use crate::{
    js::write_output::OutputNode,
    stats::{
        helpers::{get_token_as_number, get_token_as_word, num_children_more_than},
        routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use super::set_var::Scope;

pub struct Multiply {
    pub values: Vec<OutputNode>,
}

impl Multiply {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
}

impl HasReturnType for Multiply {
    fn return_type(&self) -> Vec<ReturnValType> {
        vec![ReturnValType::Number]
    }
}

impl CanValidate for Multiply {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children_more_than(n, 2)?;

        v.expect_val(ReturnValType::Number);

        for child in n.children.iter().skip(1) {
            match child.node_type {
                ZilNodeType::Token(TokenType::Number) => {
                    let num = get_token_as_number(child).unwrap();
                    self.values.push(OutputNode::Number(num));
                }
                ZilNodeType::Token(TokenType::Word) => {
                    let word = get_token_as_word(&child).unwrap();
                    if let Some(return_type) = v.has_local_var(&word) {
                        match return_type {
                            ReturnValType::Number => {
                                self.values.push(OutputNode::Variable(Scope::Local(word)));
                            }
                            _ => {
                                return Err(format!(
                                    "Variable {} is not a number\n{}",
                                    word,
                                    format_file_location(&child)
                                ));
                            }
                        }
                    } else if v.is_global(&word) {
                        self.values.push(OutputNode::Variable(Scope::Global(word)));
                    } else {
                        return Err(format!(
                            "Word {} not found in locals or globals\n{}",
                            word,
                            format_file_location(&child)
                        ));
                    }
                }
                ZilNodeType::Cluster => match v.validate_cluster(&child) {
                    Ok(_) => match v.take_last_writer() {
                        Some(w) => self.values.push(OutputNode::Writer(w)),
                        None => unreachable!(),
                    },
                    Err(e) => return Err(e),
                },
                _ => {
                    return Err(format!(
                        "Expected word, number, or cluster, found {}\n{}",
                        child.node_type,
                        format_file_location(&n)
                    ));
                }
            }
        }

        Ok(())
    }
}
