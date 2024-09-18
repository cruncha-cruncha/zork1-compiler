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

pub struct Location {
    pub scope: Scope,
}

impl Location {
    pub fn new() -> Self {
        Self { scope: Scope::TBD }
    }
}

impl HasReturnType for Location {
    fn return_type(&self) -> Vec<ReturnValType> {
        match self.scope {
            Scope::Player => vec![ReturnValType::RP],
            _ => vec![ReturnValType::Inst, ReturnValType::RP],
        }
    }
}

impl CanValidate for Location {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children(n, 2)?;

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
                } else {
                    return Err(format!(
                        "Word {} is not player, and not found in locals\n{}",
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

        Ok(())
    }
}
