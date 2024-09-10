use crate::{
    stats::{
        helpers::get_token_as_word,
        routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use super::set_var::Scope;

pub struct Description {
    pub scope: Scope,
}

impl Description {
    pub fn new() -> Self {
        Self { scope: Scope::TBD }
    }
}

impl HasReturnType for Description {
    fn return_type(&self) -> ReturnValType {
        ReturnValType::None
    }
}

impl CanValidate for Description {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        if n.children.len() != 2 {
            return Err(format!(
                "Expected 2 children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        v.expect_val(ReturnValType::Location);

        match n.children[1].node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&n.children[1]).unwrap();

                if let Some(var_type) = v.has_local_var(&word) {
                    match var_type {
                        ReturnValType::ObjectName | ReturnValType::Location => {
                            self.scope = Scope::Local(word);
                        }
                        _ => {
                            return Err(format!(
                                "Variable {} is not a local variable of Location type\n{}",
                                word,
                                format_file_location(&n.children[1])
                            ));
                        }
                    }
                } else if v.is_room(&word) {
                    self.scope = Scope::Room(word);
                } else if v.is_object(&word) {
                    self.scope = Scope::Object(word);
                } else {
                    return Err(format!(
                        "Variable {} not found in room or object table\n{}",
                        word,
                        format_file_location(&n.children[1])
                    ));
                }
            }
            ZilNodeType::Cluster => match v.validate_cluster(&n.children[1]) {
                Ok(_) => match v.take_last_writer() {
                    Some(w) => self.scope = Scope::LOC(w),
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
