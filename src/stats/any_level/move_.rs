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

// move PLAYER to a ROOM
// move an OBJECT to PLAYER, a ROOM, or another OBJECT

pub struct Move {
    pub thing: Scope,
    pub to: Scope,
}

impl Move {
    pub fn new() -> Self {
        Self {
            thing: Scope::TBD,
            to: Scope::TBD,
        }
    }
}

impl HasReturnType for Move {
    fn return_type(&self) -> ReturnValType {
        // if move was successful
        ReturnValType::Boolean
    }
}

impl CanValidate for Move {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        if n.children.len() != 3 {
            return Err(format!(
                "Expected exactly 3 children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        let second_child = &n.children[1];
        let second_word = get_token_as_word(&second_child);
        let third_child = &n.children[2];

        if second_word.is_some() && second_word.as_ref().unwrap() == "PLAYER" {
            self.thing = Scope::Local("player".to_string());
            match third_child.node_type {
                ZilNodeType::Token(TokenType::Word) => {
                    let word = get_token_as_word(&third_child).unwrap();
                    if !v.is_room(&word) {
                        return Err(format!(
                            "Expected a room, found {}\n{}",
                            word,
                            format_file_location(&third_child)
                        ));
                    }
                    self.to = Scope::Room(word);
                }
                ZilNodeType::Cluster => {
                    v.expect_val(ReturnValType::Location);
                    match v.validate_cluster(&third_child) {
                        Ok(_) => match v.take_last_writer() {
                            Some(w) => self.to = Scope::LOC(w),
                            None => unreachable!(),
                        },
                        Err(e) => return Err(e),
                    }
                }
                _ => {
                    return Err(format!(
                        "Expected a word or cluster, found {}\n{}",
                        third_child.node_type,
                        format_file_location(&third_child)
                    ));
                }
            }
            return Ok(());
        }

        match second_child.node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let second_word = second_word.unwrap();
                if let Some(var_type) = v.has_local_var(&second_word) {
                    match var_type {
                        ReturnValType::ObjectName | ReturnValType::Location => {
                            self.thing = Scope::Local(second_word.clone())
                        }
                        _ => {
                            return Err(format!(
                                "Variable {} is not player, a room, or an object\n{}",
                                second_word,
                                format_file_location(&second_child)
                            ));
                        }
                    }
                } else if let Some(_object) = v.get_object(&second_word) {
                    self.thing = Scope::Object(second_word.clone());
                } else {
                    return Err(format!(
                        "Variable {} is not player, a room, or an object\n{}",
                        second_word,
                        format_file_location(&second_child)
                    ));
                }
            }
            ZilNodeType::Cluster => {
                v.expect_val(ReturnValType::Location);
                match v.validate_cluster(&second_child) {
                    Ok(_) => match v.take_last_writer() {
                        Some(w) => self.thing = Scope::LOC(w),
                        None => unreachable!(),
                    },
                    Err(e) => return Err(e),
                }
            }
            _ => {
                return Err(format!(
                    "Expected a word or cluster, found {}\n{}",
                    second_child.node_type,
                    format_file_location(&second_child)
                ));
            }
        }

        match third_child.node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&third_child).unwrap();
                if let Some(var_type) = v.has_local_var(&word) {
                    match var_type {
                        ReturnValType::Location | ReturnValType::ObjectName => {
                            self.to = Scope::Local(word.clone())
                        }
                        _ => {
                            return Err(format!(
                                "Variable {} is not player, a room, or an object\n{}",
                                word,
                                format_file_location(&second_child)
                            ));
                        }
                    }
                } else if v.is_room(&word) {
                    self.to = Scope::Room(word);
                } else if v.is_object(&word) {
                    self.to = Scope::Object(word);
                } else {
                    return Err(format!(
                        "Expected player, room, or object, found {}\n{}",
                        word,
                        format_file_location(&third_child)
                    ));
                }
            }
            ZilNodeType::Cluster => {
                v.expect_val(ReturnValType::Location);
                match v.validate_cluster(&third_child) {
                    Ok(_) => match v.take_last_writer() {
                        Some(w) => self.to = Scope::LOC(w),
                        None => unreachable!(),
                    },
                    Err(e) => return Err(e),
                }
            }
            _ => {
                return Err(format!(
                    "Expected a word or cluster, found {}\n{}",
                    third_child.node_type,
                    format_file_location(&third_child)
                ));
            }
        }

        Ok(())
    }
}
