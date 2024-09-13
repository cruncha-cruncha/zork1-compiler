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

use super::set_var::{LocalVar, Scope};

pub struct Location {
    pub scope: Scope,
}

impl Location {
    pub fn new() -> Self {
        Self { scope: Scope::TBD }
    }
}

impl HasReturnType for Location {
    fn return_type(&self) -> ReturnValType {
        // this is always a full object in js
        ReturnValType::Location
    }
}

impl CanValidate for Location {
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
                if word == "PLAYER" {
                    self.scope = Scope::Player;
                } else if let Some(var_type) = v.has_local_var(&word) {
                    match var_type {
                        ReturnValType::Location => {
                            self.scope = Scope::Local(LocalVar {
                                name: word.to_string(),
                                return_type: var_type,
                            });
                        }
                        _ => {
                            return Err(format!(
                                "Variable {} is not player, a room, or an object\n{}",
                                word,
                                format_file_location(&n.children[1])
                            ));
                        }
                    }
                } else if v.is_object(&word) {
                    self.scope = Scope::Object(word);
                } else {
                    return Err(format!(
                        "Variable {} is a word, but not an object or local object variable\n{}",
                        word,
                        format_file_location(&n.children[1])
                    ));
                }
            }
            ZilNodeType::Cluster => match v.validate_cluster(&n.children[1]) {
                Ok(_) => match v.take_last_writer() {
                    Some(w) => {
                        self.scope = Scope::LOC(w);
                    }
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
