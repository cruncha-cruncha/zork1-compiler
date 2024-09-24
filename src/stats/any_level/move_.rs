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

// MOVE INST (aka delete it)
// MOVE PLAYER ROOM
// MOVE INST IRP
// where:
// PLAYER = PLAYER
// ROOM = any word that is a room
// INST = an object instance
// IRP = an Inst, Room, or Player
// move can fail

pub struct Move {
    pub item: Scope,
    pub destination: Option<Scope>,
}

impl Move {
    pub fn new() -> Self {
        Self {
            item: Scope::TBD,
            destination: None,
        }
    }
}

impl HasReturnType for Move {
    fn return_type(&self) -> Vec<ReturnValType> {
        // if move was successful
        vec![ReturnValType::None]
    }
}

impl CanValidate for Move {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children_between(n, 2, 3)?;

        if n.children.len() == 2 {
            // MOVE INST (aka delete it)

            let second_child = &n.children[1];
            v.expect_val(ReturnValType::Inst);

            match second_child.node_type {
                ZilNodeType::Token(TokenType::Word) => {
                    let word = get_token_as_word(&second_child).unwrap();
                    if let Some(return_type) = v.has_local_var(&word) {
                        match return_type {
                            ReturnValType::Inst => {
                                self.item = Scope::Local(word);
                            }
                            _ => {
                                return Err(format!(
                                    "Variable {} is not an object\n{}",
                                    word,
                                    format_file_location(&second_child)
                                ));
                            }
                        }
                    } else {
                        return Err(format!(
                            "Variable {} not found in locals\n{}",
                            word,
                            format_file_location(&second_child)
                        ));
                    }
                }
                ZilNodeType::Cluster => match v.validate_cluster(&second_child) {
                    Ok(_) => match v.take_last_writer() {
                        Some(w) => self.item = Scope::Writer(w),
                        None => unreachable!(),
                    },
                    Err(e) => return Err(e),
                },
                _ => {
                    return Err(format!(
                        "Expected a word or cluster, found {}\n{}",
                        second_child.node_type,
                        format_file_location(&second_child)
                    ));
                }
            }
        }

        let second_word = get_token_as_word(&n.children[1]);
        if second_word.is_ok() && second_word.as_ref().unwrap() == "PLAYER" {
            // MOVE PLAYER ROOM
            v.expect_val(ReturnValType::RP);
            self.item = Scope::Player;
            let third_child = &n.children[2];

            match third_child.node_type {
                ZilNodeType::Token(TokenType::Word) => {
                    let word = get_token_as_word(&third_child).unwrap();
                    if v.is_room(&word) {
                        self.destination = Some(Scope::Room(word));
                    } else {
                        return Err(format!(
                            "Word {} not found in rooms\n{}",
                            word,
                            format_file_location(&n.children[1])
                        ));
                    }
                }
                ZilNodeType::Cluster => match v.validate_cluster(&third_child) {
                    Ok(_) => match v.take_last_writer() {
                        Some(w) => self.destination = Some(Scope::Writer(w)),
                        None => unreachable!(),
                    },
                    Err(e) => return Err(e),
                },
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

        if n.children.len() == 3 {
            // MOVE INST IRP

            let second_child = &n.children[1];
            let third_child = &n.children[2];

            v.expect_val(ReturnValType::Inst);

            match second_child.node_type {
                ZilNodeType::Token(TokenType::Word) => {
                    let word = get_token_as_word(&second_child).unwrap();
                    if let Some(return_type) = v.has_local_var(&word) {
                        match return_type {
                            ReturnValType::Inst => {
                                self.item = Scope::Local(word);
                            }
                            _ => {
                                return Err(format!(
                                    "Variable {} is not an object\n{}",
                                    word,
                                    format_file_location(&second_child)
                                ));
                            }
                        }
                    } else {
                        return Err(format!(
                            "Variable {} not found in locals\n{}",
                            word,
                            format_file_location(&second_child)
                        ));
                    }
                }
                ZilNodeType::Cluster => match v.validate_cluster(&second_child) {
                    Ok(_) => match v.take_last_writer() {
                        Some(w) => self.item = Scope::Writer(w),
                        None => unreachable!(),
                    },
                    Err(e) => return Err(e),
                },
                _ => {
                    return Err(format!(
                        "Expected a word or cluster, found {}\n{}",
                        second_child.node_type,
                        format_file_location(&second_child)
                    ));
                }
            }

            v.expect_vals(vec![ReturnValType::Inst, ReturnValType::RP]);

            match third_child.node_type {
                ZilNodeType::Token(TokenType::Word) => {
                    let word = get_token_as_word(&third_child).unwrap();
                    if word == "PLAYER" {
                        self.destination = Some(Scope::Player);
                    } else if v.is_room(&word) {
                        self.destination = Some(Scope::Room(word));
                    } else if let Some(return_type) = v.has_local_var(&word) {
                        match return_type {
                            ReturnValType::Inst => {
                                self.destination = Some(Scope::Local(word));
                            }
                            _ => {
                                return Err(format!(
                                    "Variable {} is not an object\n{}",
                                    word,
                                    format_file_location(&third_child)
                                ));
                            }
                        }
                    } else {
                        return Err(format!(
                            "Variable {} is not player or room and not found in locals\n{}",
                            word,
                            format_file_location(&third_child)
                        ));
                    }
                }
                ZilNodeType::Cluster => match v.validate_cluster(&third_child) {
                    Ok(_) => match v.take_last_writer() {
                        Some(w) => self.destination = Some(Scope::Writer(w)),
                        None => unreachable!(),
                    },
                    Err(e) => return Err(e),
                },
                _ => {
                    return Err(format!(
                        "Expected a word or cluster, found {}\n{}",
                        third_child.node_type,
                        format_file_location(&third_child)
                    ));
                }
            }
        }

        Ok(())
    }
}
