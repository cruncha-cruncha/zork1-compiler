use crate::{
    js::write_output::{OutputNode, OutputVariable},
    stats::{
        helpers::{get_token_as_number, get_token_as_word},
        routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use super::set_var::{Scope, VarWordType};

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
    fn return_type(&self) -> ReturnValType {
        ReturnValType::Number
    }
}

impl CanValidate for Subtract {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        if n.children.len() != 3 {
            return Err(format!(
                "Expected exactly 3 children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

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
                    if let Some(var_type) = v.has_local_var(&word) {
                        match var_type {
                            ReturnValType::Number => {
                                set_value(
                                    i,
                                    OutputNode::Variable(OutputVariable {
                                        scope: Scope::Local,
                                        name: VarWordType::Literal(word),
                                    }),
                                );
                            }
                            ReturnValType::VarName => {
                                set_value(
                                    i,
                                    OutputNode::Variable(OutputVariable {
                                        scope: Scope::Local,
                                        name: VarWordType::Literal(word),
                                    }),
                                );
                            }
                            _ => {
                                return Err(format!(
                                    "Variable {} is not a numeric local variable\n{}",
                                    word,
                                    format_file_location(&n.children[1])
                                ));
                            }
                        }
                    } else if v.is_global(&word) {
                        set_value(
                            i,
                            OutputNode::Variable(OutputVariable {
                                scope: Scope::Global,
                                name: VarWordType::Literal(word),
                            }),
                        );
                    } else {
                        return Err(format!(
                            "Variable {} not found in local or global symbol table\n{}",
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
                        "Expected word, number or cluster, found {}\n{}",
                        child.node_type,
                        format_file_location(&n)
                    ));
                }
            }
        }

        Ok(())
    }
}
