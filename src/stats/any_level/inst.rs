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

// INST IRP OBJ
// INST IRP OBJ N

use super::set_var::Scope;

pub struct Inst {
    pub scope: Scope,
    pub object_name: String,
    pub nested: bool,
}

impl Inst {
    pub fn new() -> Self {
        Self {
            scope: Scope::TBD,
            object_name: String::new(),
            nested: false,
        }
    }
}

impl HasReturnType for Inst {
    fn return_type(&self) -> Vec<ReturnValType> {
        vec![ReturnValType::Inst]
    }
}

impl CanValidate for Inst {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children_between(n, 3, 4)?;

        v.expect_vals(vec![ReturnValType::Inst, ReturnValType::RP]);

        match n.children[1].node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&n.children[1]).unwrap();
                if word == "PLAYER" {
                    self.scope = Scope::Player;
                } else if let Some(return_type) = v.has_local_var(&word) {
                    match return_type {
                        ReturnValType::Inst => self.scope = Scope::Local(word),
                        _ => {
                            return Err(format!(
                                "Variable {} is not a room or object\n{}",
                                word,
                                format_file_location(&n.children[1])
                            ));
                        }
                    }
                } else if v.is_room(&word) {
                    self.scope = Scope::Room(word);
                } else {
                    return Err(format!(
                        "Word {} is not player, nor room, and not found in locals\n{}",
                        word,
                        format_file_location(&n.children[1])
                    ));
                }
            }
            ZilNodeType::Cluster => match v.validate_cluster(&n.children[1]) {
                Ok(_) => match v.take_last_writer() {
                    Some(w) => {
                        self.scope = Scope::Writer(w);
                    }
                    None => unreachable!(),
                },
                Err(e) => return Err(e),
            },
            _ => {
                return Err(format!(
                    "Expected word, or cluster, found {}\n{}",
                    n.children[1].node_type,
                    format_file_location(&n.children[1])
                ));
            }
        }

        match n.children[2].node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&n.children[2]).unwrap();
                if v.is_object(&word) {
                    self.object_name = word;
                } else {
                    return Err(format!(
                        "Word {} is not an object\n{}",
                        word,
                        format_file_location(&n.children[2])
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "Expected word, found {}\n{}",
                    n.children[1].node_type,
                    format_file_location(&n.children[2])
                ));
            }
        }

        if n.children.len() == 4 {
            let third_word = match get_token_as_word(&n.children[3]) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            if third_word == "N" {
                self.nested = true;
            } else {
                return Err(format!(
                    "Third word is not N {}\n{}",
                    third_word,
                    format_file_location(&n.children[3])
                ));
            }
        }

        Ok(())
    }
}
