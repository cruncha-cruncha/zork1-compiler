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

pub struct Return {
    // always have to return a number, even if it has no meaning
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
    fn return_type(&self) -> Vec<ReturnValType> {
        vec![ReturnValType::None]
    }
}

impl CanValidate for Return {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children(n, 2)?;

        v.expect_val(ReturnValType::Number);

        match n.children[1].node_type {
            ZilNodeType::Token(TokenType::Number) => {
                let num = get_token_as_number(&n.children[1]).unwrap();
                self.value = OutputNode::Number(num);
            }
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&n.children[1]).unwrap();
                if let Some(return_type) = v.has_local_var(&word) {
                    match return_type {
                        ReturnValType::Number => {
                            self.value = OutputNode::Variable(Scope::Local(word));
                        }
                        _ => {
                            return Err(format!(
                                "Variable {} is not a number\n{}",
                                word,
                                format_file_location(&n.children[1])
                            ));
                        }
                    }
                } else if v.is_global(&word) {
                    self.value = OutputNode::Variable(Scope::Global(word));
                } else {
                    return Err(format!(
                        "Word {} is not player, and not found in locals, globals, rooms, or objects\n{}",
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
