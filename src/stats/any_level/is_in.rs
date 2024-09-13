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

use super::set_var::{LocalVar, Scope};

pub struct IsIn {
    pub item: Scope,
    pub container: Scope,
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
    fn return_type(&self) -> ReturnValType {
        ReturnValType::Boolean
    }
}

impl CanValidate for IsIn {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        if n.children.len() < 3 || n.children.len() > 4 {
            return Err(format!(
                "Expected exactly 3 or 4 children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        let second_child = &n.children[1];
        let third_child = &n.children[2];

        v.expect_val(ReturnValType::Location);

        match second_child.node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&second_child).unwrap();
                if word == "PLAYER" {
                    self.container = Scope::Player;
                } else if let Some(var_type) = v.has_local_var(&word) {
                    match var_type {
                        ReturnValType::Location => {
                            self.container = Scope::Local(LocalVar {
                                name: word.to_string(),
                                return_type: var_type,
                            });
                        }
                        _ => {
                            return Err(format!(
                                "Variable {} is not a location-type local variable\n{}",
                                word,
                                format_file_location(&n.children[1])
                            ));
                        }
                    }
                } else if v.is_room(&word) {
                    self.container = Scope::Room(word);
                } else if v.is_object(&word) {
                    self.container = Scope::Object(word);
                } else {
                    return Err(format!(
                        "Variable {} not found in room or object table\n{}",
                        word,
                        format_file_location(&n.children[1])
                    ));
                }
            }
            ZilNodeType::Cluster => match v.validate_cluster(&second_child) {
                Ok(_) => match v.take_last_writer() {
                    Some(w) => self.container = Scope::LOC(w),
                    None => unreachable!(),
                },
                Err(e) => return Err(e),
            },
            _ => {
                return Err(format!(
                    "Expected word, number, or cluster, found {}\n{}",
                    second_child.node_type,
                    format_file_location(&n)
                ));
            }
        }

        // lol what is DRY

        match third_child.node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&third_child).unwrap();
                if let Some(var_type) = v.has_local_var(&word) {
                    match var_type {
                        ReturnValType::Location => {
                            self.item = Scope::Local(LocalVar {
                                name: word.to_string(),
                                return_type: var_type,
                            });
                        }
                        _ => {
                            return Err(format!(
                                "Variable {} is not a location-type local variable\n{}",
                                word,
                                format_file_location(&n.children[1])
                            ));
                        }
                    }
                } else if v.is_room(&word) {
                    self.item = Scope::Room(word);
                } else if v.is_object(&word) {
                    self.item = Scope::Object(word);
                } else {
                    return Err(format!(
                        "Variable {} not found in room or object table\n{}",
                        word,
                        format_file_location(&n.children[1])
                    ));
                }
            }
            ZilNodeType::Cluster => match v.validate_cluster(&third_child) {
                Ok(_) => match v.take_last_writer() {
                    Some(w) => self.item = Scope::LOC(w),
                    None => unreachable!(),
                },
                Err(e) => return Err(e),
            },
            _ => {
                return Err(format!(
                    "Expected word, number, or cluster, found {}\n{}",
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
