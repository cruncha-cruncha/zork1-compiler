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

// COPY-MOVE OBJ IRP
// COPY-MOVE INST IRP
// where:
// INST = an object instance
// IRP = an Inst, Room, or Player
// OBJ = an object
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

        // option 1 -> OBJ IRP, option 2 -> INST IRP
        let mut option = 2;

        let second_word = get_token_as_word(&n.children[1]);
        if second_word.is_ok() {
            let second_word = second_word.as_ref().unwrap();
            if v.is_object(second_word) {
                self.item = Scope::Object(second_word.clone());
                option = 1;
            }
        }

        if option == 2 {
            // COPY-MOVE INST IRP

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

        let third_child = &n.children[2];
        v.expect_vals(vec![ReturnValType::Inst, ReturnValType::RP]);

        match third_child.node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&third_child).unwrap();
                if word == "PLAYER" {
                    self.destination = Scope::Player;
                } else if v.is_room(&word) {
                    self.destination = Scope::Room(word);
                } else if let Some(return_type) = v.has_local_var(&word) {
                    match return_type {
                        ReturnValType::Inst => {
                            self.destination = Scope::Local(word);
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
                    Some(w) => self.destination = Scope::Writer(w),
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

        Ok(())
    }
}
