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

pub struct IsIn {
    pub container: Scope,
    pub item: Scope,
    pub nested: bool,
}

impl IsIn {
    pub fn new() -> Self {
        Self {
            container: Scope::TBD,
            item: Scope::TBD,
            nested: false,
        }
    }
}

impl HasReturnType for IsIn {
    fn return_type(&self) -> Vec<ReturnValType> {
        vec![ReturnValType::Boolean]
    }
}

impl CanValidate for IsIn {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children_between(n, 3, 4)?;

        let second_child = &n.children[1];
        let third_child = &n.children[2];

        v.expect_vals(vec![ReturnValType::Inst, ReturnValType::RP]);

        match second_child.node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&second_child).unwrap();
                if word == "PLAYER" {
                    self.container = Scope::Player;
                } else if let Some(return_type) = v.has_local_var(&word) {
                    match return_type {
                        ReturnValType::Inst => self.container = Scope::Local(word),
                        _ => {
                            return Err(format!(
                                "Variable {} is not an object instance\n{}",
                                word,
                                format_file_location(&second_child)
                            ));
                        }
                    }
                } else if v.is_room(&word) {
                    self.container = Scope::Room(word);
                } else {
                    return Err(format!(
                        "Word {} not player, and not found in locals or rooms\n{}",
                        word,
                        format_file_location(&n.children[1])
                    ));
                }
            }
            ZilNodeType::Cluster => match v.validate_cluster(&second_child) {
                Ok(_) => match v.take_last_writer() {
                    Some(w) => self.container = Scope::Writer(w),
                    None => unreachable!(),
                },
                Err(e) => return Err(e),
            },
            _ => {
                return Err(format!(
                    "Expected word, or cluster, found {}\n{}",
                    second_child.node_type,
                    format_file_location(&n)
                ));
            }
        }

        match third_child.node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&third_child).unwrap();
                if word == "PLAYER" {
                    self.item = Scope::Player;
                } else if let Some(return_type) = v.has_local_var(&word) {
                    match return_type {
                        ReturnValType::Inst => self.item = Scope::Local(word),
                        _ => {
                            return Err(format!(
                                "Variable {} is not an object instance\n{}",
                                word,
                                format_file_location(&third_child)
                            ));
                        }
                    }
                } else if v.is_object(&word) {
                    self.item = Scope::Object(word);
                } else {
                    return Err(format!(
                        "Word {} not player, and not found in locals or rooms\n{}",
                        word,
                        format_file_location(&n.children[1])
                    ));
                }
            }
            ZilNodeType::Cluster => match v.validate_cluster(&third_child) {
                Ok(_) => match v.take_last_writer() {
                    Some(w) => self.item = Scope::Writer(w),
                    None => unreachable!(),
                },
                Err(e) => return Err(e),
            },
            _ => {
                return Err(format!(
                    "Expected word, or cluster, found {}\n{}",
                    third_child.node_type,
                    format_file_location(&n)
                ));
            }
        }

        if n.children.len() == 4 {
            // don't care what the fourth child is, presence is enough
            self.nested = true;
        }

        Ok(())
    }
}
