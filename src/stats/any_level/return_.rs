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

pub struct Return {
    pub value: OutputNode,
}

impl Return {
    pub fn new() -> Self {
        Self {
            value: OutputNode::TBD,
        }
    }
}

impl HasReturnType for Return {
    fn return_type(&self) -> ReturnValType {
        // always have to return a number, even if it has no meaning
        ReturnValType::Number
    }
}

impl CanValidate for Return {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        if n.children.len() != 2 {
            return Err(format!(
                "Expected 2 children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        v.expect_val(ReturnValType::Number);

        match n.children[1].node_type {
            ZilNodeType::Token(TokenType::Number) => {
                let num = get_token_as_number(&n.children[1]).unwrap();
                self.value = OutputNode::Number(num);
            }
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&n.children[1]).unwrap();
                if let Some(var_type) = v.has_local_var(&word) {
                    match var_type {
                        ReturnValType::Number => {
                            self.value = OutputNode::Variable(OutputVariable {
                                scope: Scope::Local,
                                name: VarWordType::Literal(word),
                            });
                        }
                        ReturnValType::VarName => {
                            self.value = OutputNode::Variable(OutputVariable {
                                scope: Scope::Local,
                                name: VarWordType::Literal(word),
                            });
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
                    self.value = OutputNode::Variable(OutputVariable {
                        scope: Scope::Global,
                        name: VarWordType::Literal(word),
                    });
                } else {
                    return Err(format!(
                        "Variable {} not found in local or global symbol table\n{}",
                        word,
                        format_file_location(&n.children[1])
                    ));
                }
            }
            ZilNodeType::Cluster => match v.validate_cluster(&n.children[1]) {
                Ok(_) => match v.take_last_writer() {
                    Some(w) => self.value = OutputNode::Writer(w),
                    None => unreachable!(),
                },
                Err(e) => return Err(e),
            },
            _ => {
                return Err(format!(
                    "Expected number, found {}\n{}",
                    n.children[1].node_type,
                    format_file_location(&n.children[1])
                ));
            }
        }

        Ok(())
    }
}
