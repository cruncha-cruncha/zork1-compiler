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

// starts a loop
// option 1: EACH-VAL IRP (name, val) ...
// option 2: EACH-VAL VAR (val) ...
// where:
// IRP = an object instance, a room, or player
// VAR = a global or local variable of type number
// name = developer-defined word, gets the variable name as text
// if option 1: val = developer-defined word, gets the variable value
// if option 2: val = developer-defined word, gets values from 0 to VAR value

pub struct EachVal {
    pub scope: Scope,
    pub iterate: bool,
    pub first_var: String,
    pub second_var: String,
    pub body: Vec<OutputNode>,
}

impl EachVal {
    pub fn new() -> Self {
        Self {
            scope: Scope::TBD,
            iterate: false,
            first_var: String::new(),
            second_var: String::new(),
            body: Vec::new(),
        }
    }
}

impl HasReturnType for EachVal {
    fn return_type(&self) -> Vec<ReturnValType> {
        vec![ReturnValType::None]
    }
}

impl CanValidate for EachVal {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children_more_than(n, 2)?;

        v.expect_vals(vec![
            ReturnValType::Number,
            ReturnValType::Inst,
            ReturnValType::RP,
        ]);

        let second_child = &n.children[1];
        match second_child.node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&second_child).unwrap();
                if let Some(return_type) = v.has_local_var(&word) {
                    match return_type {
                        ReturnValType::Inst | ReturnValType::RP => {
                            self.scope = Scope::Local(word);
                        }
                        ReturnValType::Number => {
                            self.scope = Scope::Local(word);
                            self.iterate = true;
                        }
                        _ => {
                            return Err(format!(
                                "Variable {} doesn't have a variables\n{}",
                                word,
                                format_file_location(&n.children[1])
                            ));
                        }
                    }
                } else if word == "PLAYER" {
                    self.scope = Scope::Player;
                } else if v.is_global(&word) {
                    self.scope = Scope::Global(word);
                    self.iterate = true;
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
                Ok(return_type) => match v.take_last_writer() {
                    Some(w) => {
                        self.scope = Scope::Writer(w);
                        if return_type == ReturnValType::Number {
                            self.iterate = true;
                        }
                    }
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
        if self.iterate {
            num_children(var_group, 1)?;
        } else {
            num_children(var_group, 2)?;
        }

        if var_group.node_type != ZilNodeType::Group {
            return Err(format!(
                "Expected group, found {}\n{}",
                var_group.node_type,
                format_file_location(&var_group)
            ));
        }

        for (i, grand_child) in var_group.children.iter().enumerate() {
            let name = get_token_as_word(&grand_child)?;

            if v.is_global(&name) || v.has_local_var(&name).is_some() {
                return Err(format!(
                    "Variable {} already found in symbol table\n{}",
                    name,
                    format_file_location(&grand_child)
                ));
            }

            match i {
                0 => {
                    v.add_local_var(name.clone(), ReturnValType::Text);
                    self.first_var = name;
                }
                1 => {
                    v.add_local_var(name.clone(), ReturnValType::Number);
                    self.second_var = name;
                }
                _ => unreachable!(),
            }
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
                        format_file_location(&n)
                    ));
                }
            }
        }

        Ok(())
    }
}
