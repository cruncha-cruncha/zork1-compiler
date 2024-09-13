use crate::{
    js::write_output::OutputNode,
    stats::{
        helpers::{get_token_as_number, get_token_as_word},
        routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use super::set_var::{LocalVar, Scope};

// divides one number by another
// returns a number

pub struct Divide {
    pub dividend: OutputNode,
    pub divisor: OutputNode,
}

impl Divide {
    pub fn new() -> Self {
        Self {
            dividend: OutputNode::TBD,
            divisor: OutputNode::TBD,
        }
    }
}

impl HasReturnType for Divide {
    fn return_type(&self) -> ReturnValType {
        ReturnValType::Number
    }
}

impl CanValidate for Divide {
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
            0 => self.dividend = d,
            1 => self.divisor = d,
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
                            ReturnValType::Number | ReturnValType::VarName => {
                                set_value(
                                    i,
                                    OutputNode::Variable(Scope::Local(LocalVar {
                                        name: word.clone(),
                                        return_type: var_type,
                                    })),
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
                        set_value(i, OutputNode::Variable(Scope::Global(word)));
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
