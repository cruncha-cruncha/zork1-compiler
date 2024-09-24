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

// LOC INST
// LOC INST N

use super::set_var::Scope;

pub struct Location {
    pub instance: Scope,
    pub nested: bool,
}

impl Location {
    pub fn new() -> Self {
        Self {
            instance: Scope::TBD,
            nested: false,
        }
    }
}

impl HasReturnType for Location {
    fn return_type(&self) -> Vec<ReturnValType> {
        vec![ReturnValType::Inst, ReturnValType::RP]
    }
}

impl CanValidate for Location {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children_between(n, 2, 3)?;

        v.expect_val(ReturnValType::Inst);

        match n.children[1].node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&n.children[1]).unwrap();
                if let Some(return_type) = v.has_local_var(&word) {
                    match return_type {
                        ReturnValType::Inst => self.instance = Scope::Local(word),
                        _ => {
                            return Err(format!(
                                "Variable {} is not an object\n{}",
                                word,
                                format_file_location(&n.children[1])
                            ));
                        }
                    }
                } else {
                    return Err(format!(
                        "Word {} not found in locals\n{}",
                        word,
                        format_file_location(&n.children[1])
                    ));
                }
            }
            ZilNodeType::Cluster => match v.validate_cluster(&n.children[1]) {
                Ok(_) => match v.take_last_writer() {
                    Some(w) => {
                        self.instance = Scope::Writer(w);
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

        if n.children.len() == 3 {
            let third_word = match get_token_as_word(&n.children[2]) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            if third_word == "N" {
                self.nested = true;
            } else {
                return Err(format!(
                    "Expected third word to be N, found {}\n{}",
                    third_word,
                    format_file_location(&n.children[2])
                ));
            }
        }

        Ok(())
    }
}
