use crate::{
    js::write_output::OutputNode,
    stats::{
        helpers::{get_token_as_word, num_children, num_children_more_than},
        routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use super::set_var::Scope;

// starts a loop, over object instances
// option 1: EACH-OBJ IRP (obj) ...
// option 2: EACH-OBJ OBJ (obj) ...
// where:
// IRP = an object instance, a room, or player
// OBJ = an object
// obj = developer-defined word, gets an object instance

pub struct EachObj {
    pub scope: Scope,
    pub name_var: String,
    pub body: Vec<OutputNode>,
}

impl EachObj {
    pub fn new() -> Self {
        Self {
            scope: Scope::TBD,
            name_var: String::new(),
            body: Vec::new(),
        }
    }
}

impl HasReturnType for EachObj {
    fn return_type(&self) -> Vec<ReturnValType> {
        vec![ReturnValType::None]
    }
}

impl CanValidate for EachObj {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children_more_than(n, 2)?;

        v.expect_vals(vec![ReturnValType::Inst, ReturnValType::RP]);

        let second_child = &n.children[1];
        match second_child.node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&second_child).unwrap();
                if let Some(return_type) = v.has_local_var(&word) {
                    match return_type {
                        ReturnValType::Inst | ReturnValType::RP => {
                            self.scope = Scope::Local(word);
                        }
                        _ => {
                            return Err(format!(
                                "Variable {} doesn't have a description\n{}",
                                word,
                                format_file_location(&second_child)
                            ));
                        }
                    }
                } else if word == "PLAYER" {
                    self.scope = Scope::Player;
                } else if v.is_room(&word) {
                    self.scope = Scope::Room(word);
                } else if v.is_object(&word) {
                    self.scope = Scope::Object(word);
                } else {
                    return Err(format!(
                        "Word {} is not player, and not found in locals, globals, rooms, or objects\n{}",
                        word,
                        format_file_location(&n.children[1])
                    ));
                }
            }
            ZilNodeType::Cluster => match v.validate_cluster(&second_child) {
                Ok(_) => match v.take_last_writer() {
                    Some(w) => self.scope = Scope::Writer(w),
                    None => unreachable!(),
                },
                Err(e) => return Err(e),
            },
            _ => {
                return Err(format!(
                    "Expected word, or cluster, found {}\n{}",
                    second_child.node_type,
                    format_file_location(&n)
                ));
            }
        }

        let var_group = &n.children[2];
        num_children(var_group, 1)?;
        if var_group.node_type != ZilNodeType::Group {
            return Err(format!(
                "Expected group, found {}\n{}",
                var_group.node_type,
                format_file_location(&var_group)
            ));
        }

        {
            let grand_child = &var_group.children[0];
            let name = get_token_as_word(&grand_child)?;

            if v.is_global(&name) || v.has_local_var(&name).is_some() {
                return Err(format!(
                    "Variable {} already found in locals or globals\n{}",
                    name,
                    format_file_location(&grand_child)
                ));
            }

            v.add_local_var(name.clone(), ReturnValType::Inst);
            self.name_var = name;
        }

        v.expect_val(ReturnValType::Any);

        for child in n.children.iter().skip(3) {
            match child.node_type {
                ZilNodeType::Cluster => match v.validate_cluster(&child) {
                    Ok(_) => match v.take_last_writer() {
                        Some(w) => self.body.push(OutputNode::Writer(w)),
                        None => unreachable!(),
                    },
                    Err(e) => return Err(e),
                },
                _ => {
                    return Err(format!(
                        "Expected cluster, found {}\n{}",
                        child.node_type,
                        format_file_location(&child)
                    ));
                }
            }
        }

        Ok(())
    }
}
