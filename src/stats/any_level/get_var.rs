use crate::{
    stats::{
        helpers::{get_token_as_word, num_children_between},
        routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use super::set_var::Scope;

// if len 2: not allowed, use the local / global variable directly
// if len 3: get IRP variable
// if len 4: get variable on first object instance in IRP
// IRP = object instance, or room, or player

pub struct GetVar {
    pub scope: Scope,
    pub object: Option<String>,
    pub var: Scope,
}

impl GetVar {
    pub fn new() -> Self {
        Self {
            scope: Scope::TBD,
            object: None,
            var: Scope::TBD,
        }
    }
}

impl HasReturnType for GetVar {
    fn return_type(&self) -> Vec<ReturnValType> {
        // returns 0 if var doesn't exist
        vec![ReturnValType::Number]
    }
}

impl CanValidate for GetVar {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children_between(n, 3, 4)?;

        v.expect_vals(vec![ReturnValType::Inst, ReturnValType::RP]);

        let second_child = &n.children[1];

        match second_child.node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&second_child).unwrap();
                if let Some(return_type) = v.has_local_var(&word) {
                    match return_type {
                        ReturnValType::Inst => {
                            self.scope = Scope::Local(word);
                        }
                        _ => {
                            return Err(format!(
                                "Variable {} is not a location\n{}",
                                word,
                                format_file_location(&second_child)
                            ));
                        }
                    }
                } else if word == "PLAYER" {
                    self.scope = Scope::Player;
                } else if v.is_room(&word) {
                    self.scope = Scope::Room(word);
                } else {
                    return Err(format!(
                            "Word {} is not player, and not found in locals, globals, rooms, or objects\n{}",
                            word,
                            format_file_location(&second_child)
                        ));
                }
            }
            ZilNodeType::Cluster => match v.validate_cluster(&second_child) {
                Ok(_) => match v.take_last_writer() {
                    Some(w) => self.scope = Scope::Writer(w),
                    None => unreachable!(),
                },
                Err(e) => return Err(e),
            },
            _ => {
                return Err(format!(
                    "Expected word, or cluster, found {}\n{}",
                    second_child.node_type,
                    format_file_location(&second_child)
                ));
            }
        }

        if n.children.len() == 4 {
            let third_child = &n.children[2];
            match third_child.node_type {
                ZilNodeType::Token(TokenType::Word) => {
                    let word = get_token_as_word(&third_child).unwrap();
                    if v.is_object(&word) {
                        self.object = Some(word);
                    } else {
                        return Err(format!(
                            "Word {} not found in objects\n{}",
                            word,
                            format_file_location(&third_child)
                        ));
                    }
                }
                _ => {
                    return Err(format!(
                        "Expected a word, found {}\n{}",
                        third_child.node_type,
                        format_file_location(&third_child)
                    ));
                }
            }
        }

        let last_child = &n.children.last().unwrap();
        let last_word = get_token_as_word(last_child)?;
        if let Some(return_type) = v.has_local_var(&last_word) {
            match return_type {
                ReturnValType::Text => {
                    self.var = Scope::Local(last_word);
                }
                _ => {
                    return Err(format!(
                        "Variable {} is not a number\n{}",
                        last_word,
                        format_file_location(last_child)
                    ));
                }
            }
        } else {
            self.var = Scope::Bare(last_word.clone());
        }

        Ok(())
    }
}
