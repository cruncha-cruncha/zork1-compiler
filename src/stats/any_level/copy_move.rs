use crate::{
    stats::{
        helpers::{get_token_as_word, num_children},
        routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use super::set_var::Scope;

// copy and move an OBJECT to PLAYER, a ROOM, or another OBJECT

// TODO:
// COPY-MOVE INST INST
// COPY-MOVE INST IRP OBJ
// COPY-MOVE IRP OBJ IRP
// COPY-MOVE IRP OBJ IRP OBJ
//
// where:
// PLAYER = PLAYER
// ROOM = any word that is a room
// INST = an object instance
// IRP = an Inst, Room, or Player
// OBJ = an object
// and IRP OBJ = the first instance of OBJ in IRP
// copy-move can fail

pub struct CopyMove {
    pub item: Scope,
    pub destination: Scope,
}

impl CopyMove {
    pub fn new() -> Self {
        Self {
            item: Scope::TBD,
            destination: Scope::TBD,
        }
    }
}

impl HasReturnType for CopyMove {
    fn return_type(&self) -> Vec<ReturnValType> {
        // if copy-move was successful
        vec![ReturnValType::Boolean]
    }
}

impl CanValidate for CopyMove {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children(n, 3)?;

        v.expect_val(ReturnValType::Inst);

        let second_child = &n.children[1];
        let second_word = get_token_as_word(&second_child);
        let third_child = &n.children[2];

        match second_child.node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let second_word = second_word.unwrap();
                if let Some(return_type) = v.has_local_var(&second_word) {
                    match return_type {
                        ReturnValType::Inst => self.item = Scope::Local(second_word),
                        _ => {
                            return Err(format!(
                                "Variable {} is not a room or object\n{}",
                                second_word,
                                format_file_location(&second_child)
                            ));
                        }
                    }
                } else if v.is_object(&second_word) {
                    self.item = Scope::Object(second_word);
                } else {
                    return Err(format!(
                        "Word {} not found in locals, globals, or objects\n{}",
                        second_word,
                        format_file_location(&n.children[1])
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
                    "Expected a word, or cluster, found {}\n{}",
                    second_child.node_type,
                    format_file_location(&second_child)
                ));
            }
        }

        match third_child.node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&third_child).unwrap();
                if word == "PLAYER" {
                    self.destination = Scope::Player;
                } else if let Some(return_type) = v.has_local_var(&word) {
                    match return_type {
                        ReturnValType::Inst => self.destination = Scope::Local(word),
                        _ => {
                            return Err(format!(
                                "Variable {} is not a room or object\n{}",
                                word,
                                format_file_location(&third_child)
                            ));
                        }
                    }
                } else if v.is_room(&word) {
                    self.destination = Scope::Room(word);
                } else {
                    return Err(format!(
                        "Expected player, room, or object, found {}\n{}",
                        word,
                        format_file_location(&third_child)
                    ));
                }
            }
            ZilNodeType::Cluster => match v.validate_cluster(&third_child) {
                Ok(_) => match v.take_last_writer() {
                    Some(w) => self.destination = Scope::Writer(w),
                    None => unreachable!(),
                },
                Err(e) => return Err(e),
            },
            _ => {
                return Err(format!(
                    "Expected a word, or cluster, found {}\n{}",
                    third_child.node_type,
                    format_file_location(&third_child)
                ));
            }
        }

        Ok(())
    }
}
