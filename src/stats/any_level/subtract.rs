use crate::{
    js::write_output::OutputNode,
    stats::{
        helpers::{get_token_as_number, get_token_as_word, num_children},
        routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use super::set_var::Scope;

pub struct Subtract {
    pub minuend: OutputNode,
    pub subtrahend: OutputNode,
}

impl Subtract {
    pub fn new() -> Self {
        Self {
            minuend: OutputNode::TBD,
            subtrahend: OutputNode::TBD,
        }
    }
}

impl HasReturnType for Subtract {
    fn return_type(&self) -> Vec<ReturnValType> {
        vec![ReturnValType::Number]
    }
}

impl CanValidate for Subtract {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children(n, 3)?;

        v.expect_val(ReturnValType::Number);

        let mut set_value = |i: usize, d: OutputNode| match i {
            0 => self.minuend = d,
            1 => self.subtrahend = d,
            _ => unreachable!(),
        };

        for (i, child) in n.children.iter().skip(1).enumerate() {
            match child.node_type {
                ZilNodeType::Token(TokenType::Number) => {
                    let num = get_token_as_number(child).unwrap();
                    set_value(i, OutputNode::Number(num));
                }
                ZilNodeType::Token(TokenType::Word) => {
                    let word = get_token_as_word(&child).unwrap();
                    if let Some(return_type) = v.has_local_var(&word) {
                        match return_type {
                            ReturnValType::Number => {
                                set_value(i, OutputNode::Variable(Scope::Local(word)));
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
                        set_value(i, OutputNode::Variable(Scope::Global(word)));
                    } else {
                        return Err(format!(
                            "Variable {} not found in locals or globals\n{}",
                            word,
                            format_file_location(&child)
                        ));
                    }
                }
                ZilNodeType::Cluster => match v.validate_cluster(child) {
                    Ok(_) => match v.take_last_writer() {
                        Some(w) => set_value(i, OutputNode::Writer(w)),
                        None => unreachable!(),
                    },
                    Err(e) => return Err(e),
                },
                _ => {
                    return Err(format!(
                        "Expected number, word, or cluster, found {}\n{}",
                        child.node_type,
                        format_file_location(&n)
                    ));
                }
            }
        }

        Ok(())
    }
}
