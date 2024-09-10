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

// if length 3: get val of variable in room, object, or player
// if length 2: get val of local variable or global variable

pub struct GetVar {
    pub scope: Option<Scope>,
    pub var: Scope,
}

impl GetVar {
    pub fn new() -> Self {
        Self {
            scope: None,
            var: Scope::TBD,
        }
    }
}

impl HasReturnType for GetVar {
    fn return_type(&self) -> ReturnValType {
        // returns 0 if var doesn't exist
        ReturnValType::Number
    }
}

impl CanValidate for GetVar {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        if n.children.len() != 2 && n.children.len() != 3 {
            return Err(format!(
                "Expected 2 or 3 children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        let second_child = &n.children[1];

        if n.children.len() == 2 {
            let second_word = get_token_as_word(&second_child);
            if second_word.is_none() {
                return Err(format!(
                    "Expected word, found {}\n{}",
                    second_child.node_type,
                    format_file_location(&n)
                ));
            }
            let second_word = second_word.unwrap();

            if let Some(var_type) = v.has_local_var(&second_word) {
                match var_type {
                    ReturnValType::Number | ReturnValType::VarName => {
                        self.var = Scope::Local(second_word.clone())
                    }
                    _ => {
                        return Err(format!(
                            "Variable {} is not a numeric local variable\n{}",
                            second_word,
                            format_file_location(&second_child)
                        ));
                    }
                }
            } else if v.is_global(&second_word) {
                self.var = Scope::Global(second_word.clone());
            } else {
                return Err(format!(
                    "Variable {} not found in local or global symbol table\n{}",
                    second_word,
                    format_file_location(&second_child)
                ));
            }

            return Ok(());
        }

        if n.children.len() == 3 {
            let mut found_scope = false;

            if second_child.node_type == ZilNodeType::Cluster {
                v.expect_val(ReturnValType::Location);
                match v.validate_cluster(&second_child) {
                    Ok(_) => match v.take_last_writer() {
                        Some(w) => self.scope = Some(Scope::LOC(w)),
                        None => unreachable!(),
                    },
                    Err(e) => return Err(e),
                }
                found_scope = true;
            }

            if second_child.node_type == ZilNodeType::Token(TokenType::Word) {
                let second_word = get_token_as_word(&second_child).unwrap();
                if let Some(var_type) = v.has_local_var(&second_word) {
                    match var_type {
                        ReturnValType::ObjectName | ReturnValType::Location => {
                            self.scope = Some(Scope::Local(second_word.clone()))
                        }
                        _ => {
                            return Err(format!(
                                "Variable {} is not player, a room, or an object\n{}",
                                second_word,
                                format_file_location(&second_child)
                            ));
                        }
                    }
                    found_scope = true;
                }

                if let Some(_object) = v.get_object(&second_word) {
                    self.scope = Some(Scope::Object(second_word.clone()));
                    found_scope = true;
                }

                if let Some(_room) = v.get_room(&second_word) {
                    self.scope = Some(Scope::Room(second_word.clone()));
                    found_scope = true;
                }
            }

            if !found_scope {
                return Err(format!(
                    "Expected word or cluster, found {}\n{}",
                    second_child.node_type,
                    format_file_location(&second_child)
                ));
            }

            let third_word = get_token_as_word(&n.children[2]);
            if third_word.is_none() {
                return Err(format!(
                    "Expected word, found {}\n{}",
                    n.children[2].node_type,
                    format_file_location(&n)
                ));
            }
            let third_word = third_word.unwrap();

            match v.has_local_var(&third_word) {
                Some(rt) => match rt {
                    ReturnValType::VarName | ReturnValType::Number => {
                        self.var = Scope::Local(third_word.clone());
                    }
                    _ => {
                        return Err(format!(
                            "Variable {} is not a local numeric variable\n{}",
                            third_word,
                            format_file_location(&n.children[2])
                        ));
                    }
                },
                _ => {
                    self.var = Scope::Bare(third_word.clone());
                }
            }

            return Ok(());
        }

        Ok(())
    }
}
