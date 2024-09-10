use crate::{
    js::write_output::{OutputNode, OutputVariable},
    stats::{
        helpers::{get_token_as_number, get_token_as_text, get_token_as_word},
        routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use super::set_var::{Scope, VarWordType};

pub struct Tell {
    pub text: Vec<OutputNode>,
    pub cr: bool,
}

impl Tell {
    pub fn new() -> Self {
        Self {
            text: Vec::new(),
            cr: false,
        }
    }
}

impl HasReturnType for Tell {
    fn return_type(&self) -> ReturnValType {
        ReturnValType::None
    }
}

impl CanValidate for Tell {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        if n.children.len() < 2 {
            return Err(format!(
                "Not enough children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        v.expect_vals(vec![ReturnValType::Number, ReturnValType::Text]);

        for (i, child) in n.children.iter().skip(1).enumerate() {
            match child.node_type {
                ZilNodeType::Token(TokenType::Text) => {
                    let text = get_token_as_text(child).unwrap();
                    self.text.push(OutputNode::Text(text));
                }
                ZilNodeType::Token(TokenType::Word) => {
                    let word = get_token_as_word(child).unwrap();
                    if (i + 1 == n.children.len() - 1) && (word == "CR") {
                        self.cr = true;
                        continue;
                    }

                    if let Some(var_type) = v.has_local_var(&word) {
                        match var_type {
                            ReturnValType::Number => {
                                self.text.push(OutputNode::Variable(OutputVariable {
                                    scope: Scope::Local,
                                    name: VarWordType::Literal(word),
                                }));
                            }
                            ReturnValType::VarName => {
                                self.text.push(OutputNode::Variable(OutputVariable {
                                    scope: Scope::Local,
                                    name: VarWordType::Literal(word),
                                }));
                            }
                            _ => {
                                return Err(format!(
                                    "Variable {} is not a numeric local variable\n{}",
                                    word,
                                    format_file_location(&child)
                                ));
                            }
                        }
                    } else if v.is_global(&word) {
                        self.text.push(OutputNode::Variable(OutputVariable {
                            scope: Scope::Global,
                            name: VarWordType::Literal(word),
                        }));
                    } else {
                        return Err(format!(
                            "Variable {} not found in local or global symbol table\n{}",
                            word,
                            format_file_location(&child)
                        ));
                    }
                }
                ZilNodeType::Token(TokenType::Number) => {
                    let number = get_token_as_number(child).unwrap();
                    self.text.push(OutputNode::Number(number));
                }
                ZilNodeType::Cluster => match v.validate_cluster(child) {
                    Ok(_) => match v.take_last_writer() {
                        Some(w) => self.text.push(OutputNode::Writer(w)),
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
