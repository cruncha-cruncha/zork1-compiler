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

// MOVE PLAYER ROOM
// MOVE INST IRP
// MOVE INST IRP OBJ
// MOVE IRP OBJ IRP
// MOVE IRP OBJ IRP OBJ
//
// where:
// PLAYER = PLAYER
// ROOM = any word that is a room
// INST = an object instance
// IRP = an Inst, Room, or Player
// OBJ = an object
// and IRP OBJ = the first instance of OBJ in IRP
// move can fail

pub struct Move {
    pub item_scope: Scope,
    pub item_name: Option<String>,
    pub destination_scope: Scope,
    pub destination_name: Option<String>,
}

impl Move {
    pub fn new() -> Self {
        Self {
            item_scope: Scope::TBD,
            item_name: None,
            destination_scope: Scope::TBD,
            destination_name: None,
        }
    }
}

impl HasReturnType for Move {
    fn return_type(&self) -> Vec<ReturnValType> {
        // if move was successful
        vec![ReturnValType::Boolean]
    }
}

impl CanValidate for Move {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children_between(n, 3, 5)?;

        let second_word = get_token_as_word(&n.children[1]);

        if second_word.is_ok() && second_word.as_ref().unwrap() == "PLAYER" {
            // MOVE PLAYER ROOM
            v.expect_val(ReturnValType::RP);
            self.item_scope = Scope::Player;
            let third_child = &n.children[2];

            match third_child.node_type {
                ZilNodeType::Token(TokenType::Word) => {
                    let word = get_token_as_word(&third_child).unwrap();
                    if v.is_room(&word) {
                        self.destination_scope = Scope::Room(word);
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
                        Some(w) => self.destination_scope = Scope::Writer(w),
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
                                self.item_scope = Scope::Local(word);
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
                        Some(w) => self.item_scope = Scope::Writer(w),
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
                        self.destination_scope = Scope::Player;
                    } else if v.is_room(&word) {
                        self.destination_scope = Scope::Room(word);
                    } else if let Some(return_type) = v.has_local_var(&word) {
                        match return_type {
                            ReturnValType::Inst => {
                                self.destination_scope = Scope::Local(word);
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
                        Some(w) => self.destination_scope = Scope::Writer(w),
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

        if n.children.len() == 4 {
            // if last child is a word that is an object -> MOVE INST IRP OBJ
            // else -> MOVE IRP OBJ IRP
            let mut first_option = false;

            let second_child = &n.children[1];
            let third_child = &n.children[2];
            let fourth_child = &n.children[3];

            match get_token_as_word(&fourth_child) {
                Ok(word) => {
                    if v.is_object(&word) {
                        first_option = true;
                    }
                }
                Err(_) => {}
            }

            if first_option {
                v.expect_val(ReturnValType::Inst);

                match second_child.node_type {
                    ZilNodeType::Token(TokenType::Word) => {
                        let word = get_token_as_word(&second_child).unwrap();
                        if let Some(return_type) = v.has_local_var(&word) {
                            match return_type {
                                ReturnValType::Inst => {
                                    self.item_scope = Scope::Local(word);
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
                            Some(w) => self.item_scope = Scope::Writer(w),
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
                            self.destination_scope = Scope::Player;
                        } else if v.is_room(&word) {
                            self.destination_scope = Scope::Room(word);
                        } else if let Some(return_type) = v.has_local_var(&word) {
                            match return_type {
                                ReturnValType::Inst => {
                                    self.destination_scope = Scope::Local(word);
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
                            Some(w) => self.destination_scope = Scope::Writer(w),
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

                match fourth_child.node_type {
                    ZilNodeType::Token(TokenType::Word) => {
                        let word = get_token_as_word(&fourth_child).unwrap();
                        if v.is_object(&word) {
                            self.destination_name = Some(word);
                        } else {
                            return Err(format!(
                                "Word {} not found in objects\n{}",
                                word,
                                format_file_location(&fourth_child)
                            ));
                        }
                    }
                    _ => {
                        return Err(format!(
                            "Expected a word, found {}\n{}",
                            fourth_child.node_type,
                            format_file_location(&fourth_child)
                        ));
                    }
                }
            } else {
                v.expect_vals(vec![ReturnValType::Inst, ReturnValType::RP]);

                match second_child.node_type {
                    ZilNodeType::Token(TokenType::Word) => {
                        let word = get_token_as_word(&second_child).unwrap();
                        if word == "PLAYER" {
                            self.item_scope = Scope::Player;
                        } else if v.is_room(&word) {
                            self.item_scope = Scope::Room(word);
                        } else if let Some(return_type) = v.has_local_var(&word) {
                            match return_type {
                                ReturnValType::Inst => {
                                    self.item_scope = Scope::Local(word);
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
                                "Variable {} is not player or room and not found in locals\n{}",
                                word,
                                format_file_location(&second_child)
                            ));
                        }
                    }
                    ZilNodeType::Cluster => match v.validate_cluster(&second_child) {
                        Ok(_) => match v.take_last_writer() {
                            Some(w) => self.item_scope = Scope::Writer(w),
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

                match third_child.node_type {
                    ZilNodeType::Token(TokenType::Word) => {
                        let word = get_token_as_word(&third_child).unwrap();
                        if v.is_object(&word) {
                            self.item_name = Some(word);
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

                v.expect_vals(vec![ReturnValType::Inst, ReturnValType::RP]);

                match fourth_child.node_type {
                    ZilNodeType::Token(TokenType::Word) => {
                        let word = get_token_as_word(&fourth_child).unwrap();
                        if word == "PLAYER" {
                            self.destination_scope = Scope::Player;
                        } else if v.is_room(&word) {
                            self.destination_scope = Scope::Room(word);
                        } else if let Some(return_type) = v.has_local_var(&word) {
                            match return_type {
                                ReturnValType::Inst => {
                                    self.destination_scope = Scope::Local(word);
                                }
                                _ => {
                                    return Err(format!(
                                        "Variable {} is not an object\n{}",
                                        word,
                                        format_file_location(&fourth_child)
                                    ));
                                }
                            }
                        } else {
                            return Err(format!(
                                "Variable {} is not player or room and not found in locals\n{}",
                                word,
                                format_file_location(&fourth_child)
                            ));
                        }
                    }
                    ZilNodeType::Cluster => match v.validate_cluster(&fourth_child) {
                        Ok(_) => match v.take_last_writer() {
                            Some(w) => self.destination_scope = Scope::Writer(w),
                            None => unreachable!(),
                        },
                        Err(e) => return Err(e),
                    },
                    _ => {
                        return Err(format!(
                            "Expected a word or cluster, found {}\n{}",
                            fourth_child.node_type,
                            format_file_location(&fourth_child)
                        ));
                    }
                }
            }
        }

        if n.children.len() == 5 {
            // MOVE IRP OBJ IRP OBJ

            let mut set_name = |i: usize, d: String| match i {
                0 => self.item_name = Some(d),
                1 => self.destination_name = Some(d),
                _ => unreachable!(),
            };

            // third and fifth child must be a word that is an object
            for i in 0..2 {
                let child = &n.children[(i + 1) * 2];
                match child.node_type {
                    ZilNodeType::Token(TokenType::Word) => {
                        let word = get_token_as_word(&child).unwrap();
                        if v.is_object(&word) {
                            set_name(i, word);
                        } else {
                            return Err(format!(
                                "Word {} not found in objects\n{}",
                                word,
                                format_file_location(&child)
                            ));
                        }
                    }
                    _ => {
                        return Err(format!(
                            "Expected a word, found {}\n{}",
                            child.node_type,
                            format_file_location(&child)
                        ));
                    }
                }
            }

            v.expect_vals(vec![ReturnValType::Inst, ReturnValType::RP]);

            let mut set_scope = |i: usize, d: Scope| match i {
                0 => self.item_scope = d,
                1 => self.destination_scope = d,
                _ => unreachable!(),
            };

            // second and fourth child can be Player, Room, local Inst variable, or the result of a routine
            for i in 0..2 {
                let child = &n.children[i * 2 + 1];
                match child.node_type {
                    ZilNodeType::Token(TokenType::Word) => {
                        let word = get_token_as_word(&child).unwrap();
                        if word == "PLAYER" {
                            set_scope(i, Scope::Player);
                        } else if v.is_room(&word) {
                            set_scope(i, Scope::Room(word));
                        } else if let Some(return_type) = v.has_local_var(&word) {
                            match return_type {
                                ReturnValType::Inst => {
                                    set_scope(i, Scope::Local(word));
                                }
                                _ => {
                                    return Err(format!(
                                        "Variable {} is not an object\n{}",
                                        word,
                                        format_file_location(&child)
                                    ));
                                }
                            }
                        } else {
                            return Err(format!(
                                "Variable {} is not player or room and not found in locals\n{}",
                                word,
                                format_file_location(&child)
                            ));
                        }
                    }
                    ZilNodeType::Cluster => match v.validate_cluster(&child) {
                        Ok(_) => match v.take_last_writer() {
                            Some(w) => set_scope(i, Scope::Writer(w)),
                            None => unreachable!(),
                        },
                        Err(e) => return Err(e),
                    },
                    _ => {
                        return Err(format!(
                            "Expected a word or cluster, found {}\n{}",
                            child.node_type,
                            format_file_location(&child)
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}
